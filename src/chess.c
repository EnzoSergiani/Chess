#include "../inc/chess.h"
#include <stdio.h>
#include <stdlib.h>

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

