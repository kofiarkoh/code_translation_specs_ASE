#include <stdio.h>

int main()
{
    int r, c, x;
    int i, j;
    scanf("%d %d", &r, &c);
    int y[r+1][c+1];

    for (i=0; i<=r; i++){
        for (j=0; j<=c; j++){
            y[i][j] = 0;
        }
    }

    for (i=0; i<r; i++){
        for (j=0; j<c; j++){
            scanf(" %d", &y[i][j]);
        }
    }

    
    for (i=0; i<=r; i++){
        for (int j = 0; j <= c; j++){
            if (i<r && j<c){
                y[i][c] += y[i][j];
                y[r][j] += y[i][j];
            }
            else {
                y[r][c] += y[i][j];
            }
        }
    }

    y[r][c] /= 4;

    for (i=0; i<=r; i++){
        for (j=0; j<=c; j++){
            if (j!=c){
                printf("%d ", y[i][j]);
            }
            else {
                printf("%d\n", y[i][j]);
            }
        }
    }

    return 0;
}