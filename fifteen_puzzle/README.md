# 15 Puzzle Game 

The Proginal Task is to be found at [Rosettacode](http://rosettacode.org/wiki/15_puzzle_game)

The 15 puzzle (also called Gem Puzzle, Boss Puzzle, Game of Fifteen, Mystic Square and many others) is a sliding puzzle having 15 square tiles numbered 1–15 in a frame that is 4 tiles high and 4 tiles wide, leaving one unoccupied tile position. Tiles in the same row or column of the open position can be moved by sliding them horizontally or vertically, respectively. The goal of the puzzle is to place the tiles in numerical order.

Named for the number of tiles in the frame, the 15 puzzle may also be called a 16 puzzle, alluding to its total tile capacity. Similar names are used for different sized variants of the 15 puzzle, such as the 8 puzzle that has 8 tiles in a 3×3 frame.

The n puzzle is a classical problem for modelling algorithms involving heuristics. Commonly used heuristics for this problem include counting the number of misplaced tiles and finding the sum of the taxicab distances between each block and its position in the goal configuration. Note that both are admissible, i.e. they never overestimate the number of moves left, which ensures optimality for certain search algorithms such as A*.

![Image of Game](game.png)

&nbsp;

## Task

Implement the Fifteen Puzzle Game.

&nbsp;

## Solution

Using the MVC Pattern. Putting Model, View und Controller into different Modules/Folders.
The View will be implementet in 'Druid', the only currently reliably working gui library for Rust.
