#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <stdlib.h>

char board[71][71];
char ans[71][71][71];
int W, H;

int gt(char *x, char *y);
char *solve(int x, int y, int level, char now[]);
void solve_dp(void);

int main(void)
{
    while(1){
        int i, j;

        scanf("%d %d", &W, &H);
        if(W ==0 && H == 0)
            break;

        for(i = 0; i < H; i++)
            scanf("%s", &(board[i + 1][1]));

        for(i = 0; i <= H + 1; i++)
            board[i][0] = board[i][W + 1] = 'A';
        for(i = 0; i <= W + 1; i++)
            board[0][i] = board[H + 1][i] = 'A';

        char max[71];
        //char now[70];
        char *temp;
        max[0] = '1';
        max[1] = '\0';

        //for(i = 1; i <= H; i++)
        //    for(j = 1; j <= W; j++){
        //        if(isdigit(board[i][j])){
        //            //printf("start from (%d, %d)\n", j, i);
        //            temp = solve(j, i, 0, now);
        //            if(gt(temp, max)){
        //                strcpy(max, temp);
        //            }
        //            free(temp);
        //        }
        //    }

        solve_dp();
        for(i = 1; i <= H; i++){
            for(j = 1; j <= W; j++){
                if(isdigit(board[i][j]) && gt(ans[i][j], max)){
                    strcpy(max, ans[i][j]);
                }
            }
        }

        temp = max;
        while(*temp == '0')
            temp++;

        printf("%s\n", temp);
    }

    return 0;
}

int gt(char *x, char *y)
{
    char *px, *py;
    char *sx, *sy;
    px = x;
    py = y;
    int len_x, len_y;
    len_x = len_y = 0;
    while(*px == '0'){
        px++;
    }
    while(*py == '0'){
        py++;
    }

    sx = px;
    sy = py;
    while(*px != '\0'){
        px++;
        len_x++;
    }
    while(*py != '\0'){
        py++;
        len_y++;
    }

    if(len_x != len_y)
        return len_x > len_y;

    while(*sx == *sy){
        sx++;
        sy++;
    }

    return (*sx > *sy);
}

char *solve(int x, int y, int level, char now[])
{
    char *max, *temp;
    //printf("visit (%d, %d)\n", x, y);
    now[level] = board[y][x];
    if(isalpha(board[y][x + 1]) && isalpha(board[y + 1][x])){
        temp = malloc(sizeof(char) * level + 2);
        int i;
        for(i = 0; i <= level; i++){
            temp[i] = now[i];
        }
        temp[level + 1] = '\0';
        //printf("number: %s\n", temp);
        return temp;
    }

    max = malloc(sizeof(char) * 71);
    max[0] = '1';
    max[1] = '\0';
    if(isdigit(board[y][x + 1])){
        max = solve(x + 1, y, level + 1, now);
    }
    if(isdigit(board[y + 1][x])){
        temp = solve(x, y + 1, level + 1, now);
        if(gt(temp, max)){
            strcpy(max, temp);
            free(temp);
        }
    }

    return max;
}

void solve_dp(void)
{
    int i, j;

    for(i = H; i >= 1; i--){
        for(j = W; j >= 1; j--){
            char temp[71];
            temp[0] = board[i][j];
            temp[1] = '\0';
            if(isalpha(board[i][j]))
                continue;

            if(isalpha(board[i][j + 1]) && isalpha(board[i + 1][j])){
                ans[i][j][0] = board[i][j];
                ans[i][j][1] = '\0';
                //printf("ans[%d][%d]: %s\n", i, j, ans[i][j]);
                continue;
            }
            if(isdigit(board[i][j + 1]) && isdigit(board[i + 1][j])){
                if(gt(ans[i][j + 1], ans[i + 1][j])){
                    strcat(temp, ans[i][j + 1]);
                }else{
                    strcat(temp, ans[i + 1][j]);
                }
                strcpy(ans[i][j], temp);
                //printf("ans[%d][%d]: %s\n", i, j, ans[i][j]);
                continue;
            }
            if(isdigit(board[i][j + 1])){
                strcat(temp, ans[i][j + 1]);
                strcpy(ans[i][j], temp);
            }
            if(isdigit(board[i + 1][j])){
                strcat(temp, ans[i + 1][j]);
                strcpy(ans[i][j], temp);
            }
            //printf("ans[%d][%d]: %s\n", i, j, ans[i][j]);
        }
    }
}