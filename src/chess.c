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

/* === PRINT ========================================================== */

/**
 * \brief Print the board on the terminal.
 *
 * Print the board on the terminal 37*37 with file system and depending
 * the type and the color of the piece.
 */
void printBoard(Piece **board) {
  for (int i = 0; i < SIZE_BOARD * 4 + 5; i++) {
    printf("-");
  }
  printf("\n");
  for (int i = 0; i < SIZE_BOARD; i++) {
    printf("| %d |", SIZE_BOARD - i);
    for (int j = 0; j < SIZE_BOARD; j++) {
      switch (board[i][j].color) {
      case WHITE:
        printf(PRINT_WHITE_PIECE);
        printf(" %d ", board[i][j].type);
        printf(PRINT_RESET_COLOR);
        break;
      case BLACK:
        printf(PRINT_BLACK_PIECE);
        printf(" %d ", board[i][j].type);
        printf(PRINT_RESET_COLOR);
        break;
      case NONE_COLOR:
        printf("   ");
        break;
      }
      printf("|");
    }
    printf("\n");
    for (int k = 0; k < SIZE_BOARD * 4 + 5; k++) {
      printf("-");
    }
    printf("\n");
  }
  printf("    ");
  for (char c = 'A'; c <= 'H'; c++) {
    printf("| %c ", c);
  }
  printf("|\n    ");
  for (int k = 0; k < SIZE_BOARD * 4 + 1; k++) {
    printf("-");
  }
  printf("\n");
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
  freeBoard(board);
  // printf(PRINT_RESET_COLOR);
  return EXIT_SUCCESS;
}
