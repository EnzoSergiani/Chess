#ifndef H_CHESS
#define H_CHESS

/**
 * \file chess.h
 * \brief Header file containing declarations for the chess application.
 * \author Enzo SERGIANI
 */

/* === CONSTANTS ====================================================== */

/**
 * \brief The size of the chessboard.
 *
 * This constant defines the size of the chessboard as 8x8.
 */
#define SIZE_BOARD 8

/* === PIECE ========================================================== */

/**
 * @enum Type
 * @brief Represents the types of chess pieces.
 *
 * The enumeration includes values for different chess piece types,
 * such as pawn, bishop, knight, rook, queen, and king,
 * or there is no piece (none).
 */
enum Type {
  NONE_TYPE = 0, /**< No piece. */
  PAWN,          /**< Pawn piece. */
  BISHOP,        /**< Bishop piece. */
  KNIGHT,        /**< Knight piece. */
  ROOK,          /**< Rook piece. */
  QUEEN,         /**< Queen piece. */
  KING           /**< King piece. */
};

/**
 * \enum Color
 * \brief Represents the colors of chess pieces.
 *
 * The enumeration includes values for different colors of chess pieces,
 * such as white and black. The NONE_COLOR is used for situations where
 * color is not applicable or unspecified.
 */
enum Color {
  NONE_COLOR = 0, /**< No color. */
  WHITE,          /**< White color. */
  BLACK           /**< Black color. */
};

/**
 * \struct Piece
 * \brief Represents a chess piece with type and color.
 *
 * This structure combines the Type and Color enums to define a chess piece
 * with a specific type and color. It is used to represent pieces on the
 * chessboard in the game.
 *
 * \see Type
 * \see Color
 */
struct Piece {
  enum Type type;   /**< Type of the piece. */
  enum Color color; /**< Color of the piece. */
};

/**
 * \typedef Piece
 * \brief Typedef for the Piece struct.
 *
 * This typedef creates an alias 'Piece' for the struct defined.
 */
typedef struct Piece Piece;

/* === BOARD ========================================================== */

/**
 * \brief Allocate memory and initialize the chessboard.
 * \return The initialized chessboard.
 */
Piece **mallocBoard();

/**
 * \brief Free the memory allocated for the chessboard.
 * \param board The chessboard to be freed.
 */
void freeBoard(Piece **board);

/* === MAIN PROGRAM =================================================== */

/**
 * \brief Entry point of the chess program.
 * \return EXIT_SUCCESS if successful, EXIT_FAILURE otherwise.
 */
int chess();

#endif
