window.SIDEBAR_ITEMS = {"fn":[["evaluate","Evaluates mobility and part of the king safety on the `board` and returns score from the white color perspective (more than 0 when advantage, less than 0 when disadvantage). This evaluator does two things at once: first, counts all possible moves of knight, bishop, rook, queen (pawns and king are too slow and not very important), and second, sums how many squares around both kings are dangered by enemy side (`dangered_white_king_squares` and `dangered_black_king_squares`). This is used in the safety evaluator, to prevent calculating the same thing twice."],["evaluate_color","Evaluates mobility and `dangered_king_squares` on the `board` for the specified `color`."]]};