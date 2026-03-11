#include <stdio.h>

int n;
char grid[256][256];

int max(int y, int x);

int main(void)
{
    while (scanf("%d", &n), n != 0){
        int i, j;
        int maxi = 0;

        for (i = 0; i < n; i++) scanf("%s", grid[i]);
        for (i = 0; i < n; i++){
            for (j = 0; j < n; j++){
                if (grid[i][j] == '1'){
                    int t = max(i, j);

                    if (maxi < t) maxi = t;
                }
            }
        }
        printf("%d\n", maxi);
    }

    return 0;
}

int max(int y, int x)
{
    int maxi = 0;
    int i, j;
    int cnt;

    for (i = y, cnt = 0; i < n && grid[i][x] == '1'; i++, cnt++);
    if (maxi < cnt) maxi = cnt;
    for (j = x, cnt = 0; j < n && grid[y][j] == '1'; j++, cnt++);
    if (maxi < cnt) maxi = cnt;
    for (i = y, j = x, cnt = 0; i < n && j < n && grid[i][j] == '1'; i++, j++, cnt++);
    if (maxi < cnt) maxi = cnt;
    for (i = y, j = x, cnt = 0; i >= 0 && j < n && grid[i][j] == '1'; i--, j++, cnt++);
    if (maxi < cnt) maxi = cnt;

    return maxi;
}