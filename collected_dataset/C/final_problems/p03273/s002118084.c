/* ex7_3:
    Jun_JPN */

#include <stdio.h>
#include <stdlib.h>

int main(void){

    int row_num, col_num, i, j;
    int row_flag[100] = {0}, col_flag[100] = {0};
    char **board;

    scanf("%d %d", &row_num, &col_num); // 行数，列数の順で値を取得

    /* メモリの動的確保 */
    board = (char**)malloc(sizeof(char*)*row_num);  // まず行数分(row_num個)のchar*型ポインタ配列を動的確保
    if(board == NULL){          // 動的確保できなかったらエラー表示
        fprintf(stderr, "memory allocation error.\n");
        exit(1);
    }
    for(i=0; i<row_num; i++){   // col_num個のchar型配列を各ポインタに対して動的確保
        board[i] = (char*)malloc(sizeof(char)*col_num);
        if(board[i] == NULL){   // 動的確保できなかったらエラー表示
            fprintf(stderr, "memory allocation error.\n");
            exit(1);
        }
    }

    /* 行列boardを'.'か'#'で初期化 */
    for(i=0; i<row_num; i++){           //　まずi行を固定
        for(j=0; j<col_num; j++){       // j列を動かしていく
            scanf(" %c", &board[i][j]); // (i, j)成分を入力
        }
    }

    for(i=0; i<row_num; i++){       // まずi行を固定
        for(j=0; j<col_num; j++){   // j列を動かしていく
            if(board[i][j]=='#'){   // 少なくとも１つ'#'を含む行・列に旗を立てる（＝1を代入していく）
                row_flag[i] = 1;
                col_flag[j] = 1;
            }
        }
    }

    for(i=0; i<row_num; i++){   // まずi行を固定
        if(row_flag[i]){        // 少なくとも１つ'#'がi行に含まれているとき（＝row_flag[i]に旗が立っているとき）
            for(j=0; j<col_num; j++){   // j列を動かしていく
                if(col_flag[j]){        // col_flag[j]に旗が立っているとき
                    printf("%c", board[i][j]);
                }
                if(j==col_num-1){       // i行の最後まで調べ終えたとき
                    printf("\n");
                }
            }
        }
    }

    /* メモリの解放 */
    for(i=0; i<row_num; i++){   // メモリ確保時とは逆順で
        free(board[i]);
    }
    free(board);

    return 0;
}