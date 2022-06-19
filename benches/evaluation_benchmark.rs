use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use inanis::state::board::Bitboard;

fn evaluation_benchmark(criterion: &mut Criterion) {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let board = Bitboard::new_from_fen(fen, None, None, None, None, None).unwrap();

    criterion.bench_function("evaluation_benchmark", |bencher| bencher.iter(|| board.evaluate_without_cache()));
}

criterion_group!(benches, evaluation_benchmark);
criterion_main!(benches);
