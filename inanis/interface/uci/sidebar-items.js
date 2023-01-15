window.SIDEBAR_ITEMS = {"constant":[["AUTHOR",""],["VERSION",""]],"fn":[["disable_crash_files","Disables saving of crash files by reverting a panic hook to the default one."],["enable_crash_files","Enables saving of crash files by setting a custom panic hook."],["handle_debug","Handles `debug [on/off]` command by setting the proper flag."],["handle_go","Handles `go [parameters]` command by running a new search for a position which was set using `position` command. Supported parameters:"],["handle_isready","Handles `isready` command by waiting for the busy flag, and then printing response as fast as possible."],["handle_ponderhit","Handles `ponderhit` command by setting abort and ponder flags, which should switch a search mode from the ponder to the regular one."],["handle_position","Handles `position ...` command with the following variants:"],["handle_quit","Handles `quit` command by terminating engine process."],["handle_setoption","Handles `setoption [name] value [value]` command by creating or overwriting a `name` option with the specified `value`. Recreates tables if `Hash` or `Clear Hash` options are modified."],["handle_stop","Handles `stop` command by setting abort flag, which should stop ongoing search as fast as possible."],["handle_ucinewgame","Handles `ucinewgame` command by resetting a board state, recreating abort flag and clearing tables."],["recreate_state_tables","Recreates transposition table, pawn hashtable, killers table and history table."],["run","Entry point of the UCI (Universal Chess Interface) and command loop."]],"struct":[["UciState",""]]};