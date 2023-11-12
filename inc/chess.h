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

/* === STRUCTURE ====================================================== */

/**
 * @enum Type
 * @brief Represents the types of chess pieces.
 *
 * The enumeration includes values for different chess piece types,
 * such as pawn, bishop, knight, rook, queen, and king,
 * or there is no piece (none).
 */
enum Type { NONE_TYPE = 0, PAWN, BISHOP, KNIGHT, ROOK, QUEEN, KING };

/**
 * @enum Color
 * @brief Represents the colors of chess pieces.
 *
 * The enumeration includes values for different colors of chess pieces,
 * such as white and black. The NONE_COLOR is used for situations where
 * color is not applicable or unspecified.
 */
enum Color { NONE_COLOR = 0, WHITE, BLACK };

/**
 * @struct Piece
 * @brief Represents a chess piece with type and color.
 *
 * This structure combines the Type and Color enums to define a chess piece
 * with a specific type and color. It is used to represent pieces on the
 * chessboard in the game.
 */
typedef struct Piece {
  enum Type type;
  enum Color color;
} Piece;


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
