initSidebarItems({"fn":[["load_positions","Loads positions from the `epd_filename` and parses them into a list of [TestPosition]. Returns [Err] with a proper error message if the file couldn’t be parsed."],["run","Runs a test by performing a fixed-`depth` search for the positions loaded from the `epd_filename` file. To classify the test as successful, the last iteration has to return the correct move, or there must be at least `tries_to_confirm` search iterations in a row which returned the best move same as the expected one in the position."]],"struct":[["TestPosition",""]]});