#ifndef H_CHESS
#define H_CHESS

/* === STRUCTURE ====================================================== */
typedef struct Piece Piece;

/* === BOARD ========================================================== */
Piece **mallocBoard();
void freeBoard(Piece **board);

/* === MAIN PROGRAM =================================================== */
int chess();

#endif
