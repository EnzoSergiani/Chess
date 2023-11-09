#include "../inc/chess.h"
#include <stdio.h>
#include <stdlib.h>

/* === CONSTANTS ====================================================== */
#define SIZE_BOARD 8

/* === STRUCTURE ====================================================== */
// All types of pieces possible
enum Type { NONE_TYPE = 0, PAWN, BISHOP, KNIGHT, ROOK, QUEEN, KING };

// All colors of pieces possible
enum Color { NONE_COLOR = 0, WHITE, BLACK };

// Structure of the piece (type & color)
struct Piece {
  enum Type type;
  enum Color color;
};

/* === BOARD ========================================================== */
// Initiate the board before use
Piece **mallocBoard() {
  Piece **board = (Piece **)malloc(SIZE_BOARD * sizeof(Piece *));
  if (board == NULL) {
    return NULL;
  }
  for (int i = 0; i < SIZE_BOARD; i++) {
    board[i] = (Piece *)malloc(SIZE_BOARD * sizeof(Piece *));
    if (board[i] == NULL) {
      return NULL;
    }
    for (int j = 0; j < SIZE_BOARD; j++) {
      board[i][j].type = NONE_TYPE;
    }
  }
  return board;
}

// Free the board after no use
void freeBoard(Piece **board) {
  for (int i = 0; i < SIZE_BOARD; i++) {
    free(board[i]);
  }
  free(board);
}

