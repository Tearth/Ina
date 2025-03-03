use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::mobility::EvalAux;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::pst::*;
use crate::evaluation::safety;
use crate::evaluation::*;
use crate::state::text::fen;
use crate::state::*;
use crate::utils::panic_fast;
use crate::utils::rand;
use common::time::DateTime;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

const LEARNING_RATE: f32 = 0.1;
const K_STEP: f32 = 0.0001;
const B1: f32 = 0.9;
const B2: f32 = 0.999;
const OUTPUT_INTERVAL: i32 = 100;

pub struct TunerPosition {
    eval: i16,
    result_phase: u8,
    base_index: u32,
    coeffs_count: u8,
}

#[derive(Clone, Copy)]
pub struct TunerParameter {
    value: i16,
    min: i16,
    min_init: i16,
    max_init: i16,
    max: i16,
}

#[derive(Clone)]
pub struct TunerCoeff {
    pub data: u8,
}

impl TunerPosition {
    /// Constructs a new instance of [TunerPosition] with stored `board` and `result`.
    pub fn new(eval: i16, result: u8, phase: u8, base_index: u32, coeffs_count: u8) -> Self {
        Self { eval, result_phase: (result << 5) | phase, base_index, coeffs_count }
    }

    /// Gets result of the position (0 = white won, 1 = draw, 2 = black won).
    pub fn get_result(&self) -> u8 {
        self.result_phase >> 5
    }

    // Gets game phase of the position.
    pub fn get_phase(&self) -> u8 {
        self.result_phase & 0x1f
    }
}

impl TunerParameter {
    /// Constructs a new instance of [TunerParameter] with stored `value`, `min`, `min_init`, `max_init` and `max`.
    pub fn new(value: i16, min: i16, min_init: i16, max_init: i16, max: i16) -> Self {
        Self { value, min, min_init, max_init, max }
    }
}

impl TunerCoeff {
    /// Constructs a new instance of [TunerCoeff] with stored `value` and `phase`.
    pub fn new(value: i8, phase: usize) -> Self {
        Self { data: (((value + 32) as u8) << 1) | (phase as u8) }
    }

    /// Gets value and game phase as a tuple.
    pub fn get_data(&self) -> (i8, usize) {
        ((self.data as i8 >> 1) - 32, (self.data & 1) as usize)
    }
}

/// Runs tuner of evaluation parameters. The input file is specified by `epd_filename` with a list of positions and their expected results, and the `output_directory`
/// directory is used to store generated Rust sources with the optimized values. Use `random_values` to initialize evaluation parameters with random values, `k` to
/// set scaling constant (might be None) and `wdl_ratio` to set the ratio between WDL and eval. Multithreading is supported by `threads_count`. The tuner is implemented
/// using gradient descent and Adam optimizer. The result (Rust sources with the calculated values) are saved every iteration, and can be put directly into the code.
pub fn run(epd_filename: &str, output_directory: &str, random_values: bool, k: Option<f32>, wdl_ratio: f32, threads_count: usize) {
    println!("Loading EPD file...");

    let start_time = SystemTime::now();
    let mut weights_indices = HashSet::new();
    let mut weights = Vec::new();
    let mut gradients = Vec::new();
    let mut coeffs = Vec::new();
    let mut indices = Vec::new();

    let mut positions = match load_positions(epd_filename, &mut coeffs, &mut indices, &mut weights_indices) {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid EPD file: {}", error);
            return;
        }
    };

    positions.shrink_to_fit();
    coeffs.shrink_to_fit();
    indices.shrink_to_fit();

    let coeffs = Arc::new(coeffs);
    let indices = Arc::new(indices);
    println!("Loaded {} positions in {} seconds, starting tuner", positions.len(), (start_time.elapsed().unwrap().as_millis() as f32) / 1000.0);

    let mut tuner_params = load_values(random_values);

    for parameter in &tuner_params {
        weights.push(parameter.value as f32);
    }
    gradients.resize(weights.len(), 0.0);

    let mut m = Vec::new();
    m.resize(weights.len(), 0.0);

    let mut v = Vec::new();
    v.resize(weights.len(), 0.0);

    let mut weights_enabled = Vec::new();
    weights_enabled.resize(weights.len(), false);

    for i in 0..weights_enabled.len() {
        weights_enabled[i] = i == 5 || weights_indices.contains(&(i as u16));
    }

    drop(weights_indices);

    let k = k.unwrap_or_else(|| calculate_k(&positions, &coeffs, &indices, &weights, wdl_ratio, threads_count));
    let mut last_error = calculate_error(&positions, &coeffs, &indices, &weights, k, wdl_ratio, threads_count);
    let mut iterations_count = 0;

    println!("Scaling constant: {}", k);

    let mut start_time = SystemTime::now();
    loop {
        gradients.fill(0.0);

        // Calculate gradients for all coefficients
        thread::scope(|scope| {
            let mut threads = Vec::new();
            let weights = Arc::new(weights.clone());

            for chunk in positions.chunks_exact(positions.len() / threads_count) {
                let weights = weights.clone();
                let coeffs = coeffs.clone();
                let indices = indices.clone();

                threads.push(scope.spawn(move || {
                    let mut gradients = vec![0.0; weights.len()];
                    for position in chunk {
                        let position_result = position.get_result() as f32 / 2.0;
                        let position_phase = position.get_phase() as f32 / INITIAL_GAME_PHASE as f32;
                        let eval = evaluate_position(position, &coeffs, &indices, &weights);

                        let sig = sigmoid(eval, k);
                        let a = ((1.0 - wdl_ratio) * sigmoid(position.eval as f32, k) + wdl_ratio * position_result) - sig;
                        let b = sig * (1.0 - sig);

                        for i in 0..position.coeffs_count {
                            let index = position.base_index as usize + i as usize;

                            // Skip material weights
                            if indices[index] < 6 {
                                continue;
                            }

                            let (value, phase) = coeffs[index].get_data();
                            let phase = if phase == OPENING { position_phase } else { 1.0 - position_phase };
                            let c = phase * value as f32;

                            gradients[indices[index] as usize] += a * b * c;
                        }
                    }

                    gradients
                }));
            }

            for thread in threads {
                for (i, gradient) in thread.join().unwrap().iter().enumerate() {
                    gradients[i] += gradient;
                }
            }
        });

        // Apply gradients and calculate new weights but exclude material ones
        for i in 6..weights.len() {
            if weights_enabled[i] {
                let gradient = -2.0 * gradients[i] / positions.len() as f32;
                m[i] = B1 * m[i] + (1.0 - B1) * gradient;
                v[i] = B2 * v[i] + (1.0 - B2) * gradient.powi(2);

                weights[i] -= LEARNING_RATE * m[i] / (v[i] + 0.00000001).sqrt();
                weights[i] = weights[i].clamp(tuner_params[i].min as f32, tuner_params[i].max as f32);
            } else {
                weights[i] = f32::MIN;
            }
        }

        if iterations_count % OUTPUT_INTERVAL == 0 {
            for i in 0..tuner_params.len() {
                tuner_params[i].value = weights[i].round() as i16;
            }

            let mut weights_iter = weights.iter().skip(6);
            let error = calculate_error(&positions, &coeffs, &indices, &weights, k, wdl_ratio, threads_count);

            write_evaluation_parameters(&mut weights_iter, output_directory, error, k, wdl_ratio);
            write_piece_square_table(&mut weights_iter, output_directory, error, k, wdl_ratio, "PAWN", PIECE_VALUES[PAWN]);
            write_piece_square_table(&mut weights_iter, output_directory, error, k, wdl_ratio, "KNIGHT", PIECE_VALUES[KNIGHT]);
            write_piece_square_table(&mut weights_iter, output_directory, error, k, wdl_ratio, "BISHOP", PIECE_VALUES[BISHOP]);
            write_piece_square_table(&mut weights_iter, output_directory, error, k, wdl_ratio, "ROOK", PIECE_VALUES[ROOK]);
            write_piece_square_table(&mut weights_iter, output_directory, error, k, wdl_ratio, "QUEEN", PIECE_VALUES[QUEEN]);
            write_piece_square_table(&mut weights_iter, output_directory, error, k, wdl_ratio, "KING", 0);

            if weights_iter.next().is_some() {
                panic_fast!("Weights iterator has not ended properly");
            }

            println!(
                "Iteration {} done in {} seconds, error reduced from {:.6} to {:.6} ({:.6})",
                iterations_count,
                (start_time.elapsed().unwrap().as_millis() as f32) / 1000.0,
                last_error,
                error,
                last_error - error
            );

            last_error = error;
            start_time = SystemTime::now();
        }

        iterations_count += 1;
    }
}

/// Calculates scaling constant `k` that minimizes evaluation error for `positions`, `coeffs`, `indices`, `weights` and `wdl_ratio`.
/// Multithreading is supported by `threads_count`.
fn calculate_k(positions: &[TunerPosition], coeffs: &[TunerCoeff], indices: &[u16], weights: &[f32], wdl_ratio: f32, threads_count: usize) -> f32 {
    let mut k = 0.0;
    let mut last_error = f32::MAX;

    loop {
        let error = calculate_error(positions, coeffs, indices, weights, k + K_STEP, wdl_ratio, threads_count);
        if error >= last_error {
            break;
        }

        last_error = error;
        k += K_STEP;
    }

    k
}

/// Calculates an error for `positions`, `coeffs`, `indices`, `weights`, scaling constant `k` and `wdl_ratio`. Multithreading is supported by `threads_count`.
fn calculate_error(positions: &[TunerPosition], coeffs: &[TunerCoeff], indices: &[u16], weights: &[f32], k: f32, wdl_ratio: f32, threads_count: usize) -> f32 {
    let mut sum_of_errors = 0.0;
    let positions_count = positions.len();

    thread::scope(|scope| {
        let mut threads = Vec::new();
        let weights = Arc::new(weights);

        for chunk in positions.chunks_exact(positions_count / threads_count) {
            let weights = weights.clone();
            threads.push(scope.spawn(move || {
                let mut error = 0.0;
                for position in chunk {
                    let result = position.get_result() as f32 / 2.0;
                    let eval = evaluate_position(position, coeffs, indices, &weights);

                    error += (((1.0 - wdl_ratio) * sigmoid(position.eval as f32, k) + wdl_ratio * result) - sigmoid(eval, k)).powi(2);
                }

                error
            }));
        }

        for thread in threads {
            sum_of_errors += thread.join().unwrap();
        }
    });

    sum_of_errors / (positions_count as f32)
}

/// Evaluates a single `position` based on `coeffs`, `indices`, `weights`, and returns a score (casted to float).
fn evaluate_position(position: &TunerPosition, coeffs: &[TunerCoeff], indices: &[u16], weights: &[f32]) -> f32 {
    let mut opening_score = 0.0;
    let mut ending_score = 0.0;
    let position_phase = position.get_phase() as f32 / INITIAL_GAME_PHASE as f32;

    for i in 0..position.coeffs_count {
        let index = position.base_index as usize + i as usize;
        let (value, phase) = coeffs[index].get_data();
        let value = weights[indices[index] as usize] * value as f32;

        if indices[index] < 6 {
            opening_score += value;
            ending_score += value;
        } else {
            if phase == OPENING {
                opening_score += value;
            } else {
                ending_score += value;
            }
        }
    }

    (opening_score * position_phase) + (ending_score * (1.0 - position_phase))
}

/// Gets simplified sigmoid function for `e` and scaling constant `k`.
fn sigmoid(e: f32, k: f32) -> f32 {
    1.0 / (1.0 + (-k * e).exp())
}

/// Loads positions from the `epd` and parses them into a list of [TunerPosition], while filling `coeffs`, `indices` and `weights_indices`.
/// Returns [Err] with a proper error message if the file couldn't be parsed.
fn load_positions(epd: &str, coeffs: &mut Vec<TunerCoeff>, indices: &mut Vec<u16>, weights_indices: &mut HashSet<u16>) -> Result<Vec<TunerPosition>, String> {
    let mut positions = Vec::new();
    let file = match File::open(epd) {
        Ok(value) => value,
        Err(error) => return Err(format!("Invalid EPD file: {}", error)),
    };

    for line in BufReader::new(file).lines() {
        let position = line.unwrap();
        let parsed_epd = fen::epd_to_board(position.as_str())?;

        if parsed_epd.comment.is_none() {
            return Err("Game result not found".to_string());
        }

        let comment = parsed_epd.comment.unwrap();
        let comment_tokens = comment.split('|').collect::<Vec<&str>>();
        let eval = (comment_tokens[0].parse::<f32>().unwrap() * 100.0) as i16;
        let result = match comment_tokens[1] {
            "0-1" => 0,
            "1/2-1/2" => 1,
            "1-0" => 2,
            _ => return Err(format!("Invalid game result: comment_tokens[1]={}", comment)),
        };

        let mut index = 0;
        let base_index = coeffs.len();
        let mut white_aux = EvalAux::default();
        let mut black_aux = EvalAux::default();

        material::get_coeffs(&parsed_epd.board, &mut index, coeffs, indices);
        mobility::get_coeffs(&parsed_epd.board, &mut white_aux, &mut black_aux, &mut index, coeffs, indices);
        pawns::get_coeffs(&parsed_epd.board, &mut index, coeffs, indices);
        safety::get_coeffs(&parsed_epd.board, &white_aux, &black_aux, &mut index, coeffs, indices);
        pst::get_coeffs(&parsed_epd.board, PAWN, &mut index, coeffs, indices);
        pst::get_coeffs(&parsed_epd.board, KNIGHT, &mut index, coeffs, indices);
        pst::get_coeffs(&parsed_epd.board, BISHOP, &mut index, coeffs, indices);
        pst::get_coeffs(&parsed_epd.board, ROOK, &mut index, coeffs, indices);
        pst::get_coeffs(&parsed_epd.board, QUEEN, &mut index, coeffs, indices);
        pst::get_coeffs(&parsed_epd.board, KING, &mut index, coeffs, indices);

        for i in base_index..coeffs.len() {
            weights_indices.insert(indices[i]);
        }

        positions.push(TunerPosition::new(eval, result, parsed_epd.board.game_phase, base_index as u32, (coeffs.len() - base_index) as u8));
    }

    Ok(positions)
}

/// Transforms the current evaluation values into a list of [TunerParameter]. Use  `random_values` if the parameters should have
/// random values (useful when initializing tuner).
fn load_values(random_values: bool) -> Vec<TunerParameter> {
    let mut params = vec![
        TunerParameter::new(PIECE_VALUES[PAWN], PIECE_VALUES[PAWN], PIECE_VALUES[PAWN], PIECE_VALUES[PAWN], PIECE_VALUES[PAWN]),
        TunerParameter::new(PIECE_VALUES[KNIGHT], PIECE_VALUES[KNIGHT], PIECE_VALUES[KNIGHT], PIECE_VALUES[KNIGHT], PIECE_VALUES[KNIGHT]),
        TunerParameter::new(PIECE_VALUES[BISHOP], PIECE_VALUES[BISHOP], PIECE_VALUES[BISHOP], PIECE_VALUES[BISHOP], PIECE_VALUES[BISHOP]),
        TunerParameter::new(PIECE_VALUES[ROOK], PIECE_VALUES[ROOK], PIECE_VALUES[ROOK], PIECE_VALUES[ROOK], PIECE_VALUES[ROOK]),
        TunerParameter::new(PIECE_VALUES[QUEEN], PIECE_VALUES[QUEEN], PIECE_VALUES[QUEEN], PIECE_VALUES[QUEEN], PIECE_VALUES[QUEEN]),
        TunerParameter::new(PIECE_VALUES[KING], PIECE_VALUES[KING], PIECE_VALUES[KING], PIECE_VALUES[KING], PIECE_VALUES[KING]),
        TunerParameter::new(params::BISHOP_PAIR.get_opening(), -99, 10, 40, 99),
        TunerParameter::new(params::BISHOP_PAIR.get_ending(), -99, 10, 40, 99),
        TunerParameter::new(params::PAWNS_ATTACKING_PIECES.get_opening(), -99, 10, 40, 99),
        TunerParameter::new(params::PAWNS_ATTACKING_PIECES.get_ending(), -99, 10, 40, 99),
    ];

    params.append(&mut params::ROOK_OPEN_FILE.to_tuner_params(-999, 10, 40, 999, 0).to_vec());
    params.append(&mut params::ROOK_SEMI_OPEN_FILE.to_tuner_params(-999, 10, 40, 999, 0).to_vec());
    params.append(&mut params::MOBILITY_INNER.iter().flat_map(|v| v.to_tuner_params(0, 2, 6, 99, 0)).collect());
    params.append(&mut params::MOBILITY_OUTER.iter().flat_map(|v| v.to_tuner_params(0, 2, 6, 99, 0)).collect());
    params.append(&mut params::DOUBLED_PAWN.iter().flat_map(|v| v.to_tuner_params(-999, -40, -10, 999, 0)).collect());
    params.append(&mut params::ISOLATED_PAWN.iter().flat_map(|v| v.to_tuner_params(-999, -40, -10, 999, 0)).collect());
    params.append(&mut params::CHAINED_PAWN.iter().flat_map(|v| v.to_tuner_params(-999, 10, 40, 999, 0)).collect());
    params.append(&mut params::PASSED_PAWN.iter().flat_map(|v| v.to_tuner_params(-999, 10, 40, 999, 0)).collect());
    params.append(&mut params::BACKWARD_PAWN_OPEN_FILE.iter().flat_map(|v| v.to_tuner_params(-999, 10, 40, 999, 0)).collect());
    params.append(&mut params::BACKWARD_PAWN_CLOSED_FILE.iter().flat_map(|v| v.to_tuner_params(-999, 10, 40, 999, 0)).collect());
    params.append(&mut params::PAWN_SHIELD.iter().flat_map(|v| v.to_tuner_params(-999, 10, 40, 999, 0)).collect());
    params.append(&mut params::PAWN_SHIELD_OPEN_FILE.iter().flat_map(|v| v.to_tuner_params(-999, -40, -10, 999, 0)).collect());
    params.append(&mut params::KING_AREA_THREATS.iter().flat_map(|v| v.to_tuner_params(-999, -40, 40, 999, 0)).collect());
    params.append(&mut params::KNIGHT_SAFE_CHECKS.iter().flat_map(|v| v.to_tuner_params(-999, -40, 40, 999, 0)).collect());
    params.append(&mut params::BISHOP_SAFE_CHECKS.iter().flat_map(|v| v.to_tuner_params(-999, -40, 40, 999, 0)).collect());
    params.append(&mut params::ROOK_SAFE_CHECKS.iter().flat_map(|v| v.to_tuner_params(-999, -40, 40, 999, 0)).collect());
    params.append(&mut params::QUEEN_SAFE_CHECKS.iter().flat_map(|v| v.to_tuner_params(-999, -40, 40, 999, 0)).collect());

    for pov in ALL_POVS {
        for bucket in &pst::PAWN_PST_PATTERN[pov] {
            params.append(&mut bucket.iter().flat_map(|v| v.to_tuner_params(-9999, 50, 150, 9999, if pov == US { -PIECE_VALUES[PAWN] } else { 0 })).collect());
        }
    }

    for pov in ALL_POVS {
        let offset = if pov == US { -PIECE_VALUES[KNIGHT] } else { 0 };
        for bucket in &pst::KNIGHT_PST_PATTERN[pov] {
            params.append(&mut bucket.iter().flat_map(|v| v.to_tuner_params(-9999, 300, 500, 9999, offset)).collect());
        }
    }

    for pov in ALL_POVS {
        let offset = if pov == US { -PIECE_VALUES[BISHOP] } else { 0 };
        for bucket in &pst::BISHOP_PST_PATTERN[pov] {
            params.append(&mut bucket.iter().flat_map(|v| v.to_tuner_params(-9999, 300, 500, 9999, offset)).collect());
        }
    }
    for pov in ALL_POVS {
        let offset = if pov == US { -PIECE_VALUES[ROOK] } else { 0 };
        for bucket in &pst::ROOK_PST_PATTERN[pov] {
            params.append(&mut bucket.iter().flat_map(|v| v.to_tuner_params(-9999, 400, 600, 9999, offset)).collect());
        }
    }

    for pov in ALL_POVS {
        let offset = if pov == US { -PIECE_VALUES[QUEEN] } else { 0 };
        for bucket in &pst::QUEEN_PST_PATTERN[pov] {
            params.append(&mut bucket.iter().flat_map(|v| v.to_tuner_params(-9999, 800, 1400, 9999, offset)).collect());
        }
    }

    for pov in ALL_POVS {
        for bucket in &pst::KING_PST_PATTERN[pov] {
            params.append(&mut bucket.iter().flat_map(|v| v.to_tuner_params(-999, -40, 40, 999, 0)).collect());
        }
    }

    if random_values {
        rand::seed(common::time::get_unix_timestamp());
        for parameter in &mut params {
            parameter.value = rand::i16(parameter.min_init..=parameter.max_init);
        }
    }

    for parameter in &mut params {
        parameter.value = parameter.value.clamp(parameter.min, parameter.max);
    }

    params
}

/// Generates `params.rs` file with current `weights` and metadata (`best_error`, scaling constant `k`, `wdl_ratio`), then saves it into the `output_directory`.
fn write_evaluation_parameters<'a, W>(weights: &mut W, output_directory: &str, best_error: f32, k: f32, wdl_ratio: f32)
where
    W: Iterator<Item = &'a f32>,
{
    let mut output = String::new();

    output.push_str(get_header(best_error, k, wdl_ratio).as_str());
    output.push('\n');
    output.push_str("use super::*;\n");
    output.push('\n');
    output.push_str(get_constant("TEMPO", params::TEMPO).as_str());
    output.push_str(get_parameter("BISHOP_PAIR", weights).as_str());
    output.push_str(get_parameter("PAWNS_ATTACKING_PIECES", weights).as_str());
    output.push_str(get_parameter("ROOK_OPEN_FILE", weights).as_str());
    output.push_str(get_parameter("ROOK_SEMI_OPEN_FILE", weights).as_str());
    output.push_str(get_array("MOBILITY_INNER", weights, 6).as_str());
    output.push_str(get_array("MOBILITY_OUTER", weights, 6).as_str());
    output.push_str(get_array("DOUBLED_PAWN", weights, 8).as_str());
    output.push_str(get_array("ISOLATED_PAWN", weights, 8).as_str());
    output.push_str(get_array("CHAINED_PAWN", weights, 8).as_str());
    output.push_str(get_array("PASSED_PAWN", weights, 8).as_str());
    output.push_str(get_array("BACKWARD_PAWN_OPEN_FILE", weights, 8).as_str());
    output.push_str(get_array("BACKWARD_PAWN_CLOSED_FILE", weights, 8).as_str());
    output.push_str(get_array("PAWN_SHIELD", weights, 8).as_str());
    output.push_str(get_array("PAWN_SHIELD_OPEN_FILE", weights, 8).as_str());
    output.push_str(get_array("KING_AREA_THREATS", weights, 8).as_str());
    output.push_str(get_array("KNIGHT_SAFE_CHECKS", weights, 8).as_str());
    output.push_str(get_array("BISHOP_SAFE_CHECKS", weights, 8).as_str());
    output.push_str(get_array("ROOK_SAFE_CHECKS", weights, 8).as_str());
    output.push_str(get_array("QUEEN_SAFE_CHECKS", weights, 8).as_str());

    let path = Path::new(output_directory);
    fs::create_dir_all(path).unwrap();

    let path = path.join("params.rs");
    write!(&mut File::create(path).unwrap(), "{}", output).unwrap();
}

/// Generates piece-square tables as Rust source file `name`.rs with current `weights` and metadata (`best_error`, scaling constant `k`, `wdl_ratio`),
/// then saves it into the `output_directory`.
fn write_piece_square_table<'a, W>(weights: &mut W, output_directory: &str, best_error: f32, k: f32, wdl_ratio: f32, name: &str, piece_value: i16)
where
    W: Iterator<Item = &'a f32>,
{
    let mut output = String::new();

    output.push_str(get_header(best_error, k, wdl_ratio).as_str());
    output.push('\n');
    output.push_str("use super::*;\n");
    output.push('\n');
    output.push_str("#[rustfmt::skip]\n");
    output.push_str(&format!("pub const {}_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =\n", name));
    output.push_str("[\n");

    for pov in ALL_POVS {
        output.push_str("    [\n");

        for _ in 0..KING_BUCKETS_COUNT {
            output.push_str("        [\n");
            output.push_str(get_piece_square_table(weights, if pov == US { piece_value } else { 0 }).as_str());
            output.push_str("        ],\n");
        }

        output.push_str("    ],\n");
    }

    output.push_str("];\n");

    let path = Path::new(output_directory).join("pst");
    fs::create_dir_all(path).unwrap();

    let path = Path::new(output_directory).join("pst").join(format!("{}.rs", name.to_lowercase()));
    write!(&mut File::create(path).unwrap(), "{}", output).unwrap();
}

/// Gets a generated Rust source file header with timestamp, `best_error`, scaling constant `k` and `wdl_ratio`.
fn get_header(best_error: f32, k: f32, wdl_ratio: f32) -> String {
    let mut output = String::new();
    let datetime = DateTime::now();
    let datetime = format!("{:0>2}-{:0>2}-{} {:0>2}:{:0>2}:{:0>2}", datetime.day, datetime.month, datetime.year, datetime.hour, datetime.minute, datetime.day);

    output.push_str("// ------------------------------------------------------------------------- //\n");
    output.push_str(format!("// Generated at {} UTC (e = {:.6}, k = {:.4}, r = {:.2}) //\n", datetime, best_error, k, wdl_ratio).as_str());
    output.push_str("// ------------------------------------------------------------------------- //\n");
    output
}

/// Gets a Rust representation of `weight` with the specific `length` as an array `name`.
fn get_array<'a, W>(name: &str, weights: &mut W, length: usize) -> String
where
    W: Iterator<Item = &'a f32>,
{
    let mut output = format!("pub const {}: [PackedEval; {}] = [", name, length);
    for i in 0..length {
        if i > 0 {
            output += ", ";
        }

        let opening_score = *weights.next().unwrap();
        let opening_score = if opening_score != f32::MIN { opening_score.round() } else { 0.0 };

        let ending_score = *weights.next().unwrap();
        let ending_score = if ending_score != f32::MIN { ending_score.round() } else { 0.0 };

        output += &format!("s!({}, {})", opening_score, ending_score);
    }

    output += "];\n";
    output
}

/// Gets a Rust representation of the constant value.
fn get_constant(name: &str, value: i16) -> String {
    format!("pub const {}: i16 = {};\n", name, value)
}

/// Gets a Rust representation of the single `weight`.
fn get_parameter<'a, W>(name: &str, weights: &mut W) -> String
where
    W: Iterator<Item = &'a f32>,
{
    let opening_score = *weights.next().unwrap();
    let opening_score = if opening_score != f32::MIN { opening_score.round() } else { 0.0 };

    let ending_score = *weights.next().unwrap();
    let ending_score = if ending_score != f32::MIN { ending_score.round() } else { 0.0 };

    format!("pub const {}: PackedEval = s!({}, {});\n", name, opening_score, ending_score)
}

/// Gets a Rust representation of the piece-square tables with `weights` adjusted by `piece_value`.
fn get_piece_square_table<'a, W>(weights: &mut W, piece_value: i16) -> String
where
    W: Iterator<Item = &'a f32>,
{
    let mut output = String::new();
    output.push_str("            ");

    for index in ALL_SQUARES {
        let opening_score = *weights.next().unwrap();
        let opening_score = if opening_score != f32::MIN { opening_score + piece_value as f32 } else { 0.0 };

        let ending_score = *weights.next().unwrap();
        let ending_score = if ending_score != f32::MIN { ending_score + piece_value as f32 } else { 0.0 };

        output.push_str(format!("s!({:4}, {:4})", opening_score.round(), ending_score.round()).as_str());
        if index % 8 == 7 {
            output.push_str(",\n");
            if index != 63 {
                output.push_str("            ");
            }
        } else {
            output.push_str(", ");
        }
    }

    output
}
