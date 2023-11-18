/**
 * \file chess.c
 * \brief Source file for the chess game
 * \author Enzo SERGIANI
 */

#include "../inc/chess.h"
#include <stdio.h>
#include <stdlib.h>

/* === SAVE & LOAD ==================================================== */

Piece **LoadPosition(Piece **board, const char *FEN) { return board; }

int SavePosition(Piece **board) { return EXIT_SUCCESS; }

/* === BOARD ========================================================== */

/**
 * \brief Allocate memory and initialize the chessboard.
 *
 * This function dynamically allocates memory for the chessboard and initializes
 * it with default values.
 * \return The initialized chessboard.
 */
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

/**
 * \brief Free the memory allocated for the chessboard.
 * \param board The chessboard to be freed.
 */
void freeBoard(Piece **board) {
  for (int i = 0; i < SIZE_BOARD; i++) {
    free(board[i]);
  }
  free(board);
}

/* === MAIN PROGRAM =================================================== */

/**
 * \brief Entry point of the chess program.
 *
 * This function initializes the chessboard, prints it to the console, and
 * releases allocated memory.
 * \return EXIT_SUCCESS if successful, EXIT_FAILURE otherwise.
 */
int chess() {
  Piece **board = mallocBoard();
  if (board == NULL) {
    return EXIT_FAILURE;
  }
  printBoard(board);
  freeBoard(board);
  printf(PRINT_RESET_COLOR);
  return EXIT_SUCCESS;
}
