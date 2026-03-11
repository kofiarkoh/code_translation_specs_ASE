#include <stdio.h>

int num[9][9];
int flag[9][9];

void check_box(int y, int x)
{
    int i, j;
    int cnt[10] = {0};

    for (i = y; i < y + 3; i++){
        for (j = x; j < x + 3; j++){
            cnt[num[i][j]]++;
        }
    }
    for (i = y; i < y + 3; i++){
        for (j = x; j < x + 3; j++){
            if (cnt[num[i][j]] != 1){
                flag[i][j] = 1;
            }
        }
    }
}

int main(void)
{
    int n;

    scanf("%d", &n);
    while (n-- > 0){
        int num[9][9];
        int flag[9][9] = {{0}};
        int i, j;

        for (i = 0; i < 9; i++){
            int cnt[10] = {0};
            for (j = 0; j < 9; j++){
                scanf("%d", num[i] + j);
                cnt[num[i][j]]++;
            }
            for (j = 0; j < 9; j++){
                if (cnt[num[i][j]] != 1){
                    flag[i][j] = 1;
                }
            }
        }

        for (j = 0; j < 9; j++){
            int cnt[10] = {0};
            for (i = 0; i < 9; i++){
                cnt[num[i][j]]++;
            }
            for (i = 0; i < 9; i++){
                if (cnt[num[i][j]] != 1){
                    flag[i][j] = 1;
                }
            }
        }

        for (i = 0; i < 3; i++){
            for (j = 0; j < 3; j++){
                check_box(i * 3, j * 3);
            }
        }

        for (i = 0; i < 9; i++){
            for (j = 0; j < 9; j++){
                printf("%c%d", flag[i][j] ? '*' : ' ', num[i][j]);
            }
            puts("");
        }
        puts("");
    }

    return (0);
}