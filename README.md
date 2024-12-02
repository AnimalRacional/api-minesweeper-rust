# olc-rust-minesweeper
A web API that allows running minesweeper games.
2 endpoints:
## /new_game/<xsize>/<ysize>/<bomb_amount>:
Starts a new game, and returns its id
## /action/<id>/<action>/<xpos>/<ypos>
Performs one of two actions in the game with the specified id:
* reveal -> reveals a square
* flag -> flags a square
If any other action is specified, does nothing, but still returns the game state.
The game board is a list of lists of numbers, meaning the following:
* 0...8 -> the number of mines around the square
* 9 -> the square is not revealed
* 10 -> the square is flagged
* 11 -> the bomb that ended the game
