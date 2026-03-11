#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(void){
    int H, W, D;

    scanf("%d%d%d", &H, &W, &D);

    int **masu;

    masu = (int **) malloc(sizeof(int *) * H);

    int i, j;

    for(i = 0; i < H; i ++){
        masu[i] = (int *) malloc(sizeof(int) * W);

        for(j = 0; j < W; j ++){
            scanf("%d", masu[i] + j);
        }
    }
    int *masu_x = (int *) malloc(sizeof(int) * (H * W + 1));
    int *masu_y = (int *) malloc(sizeof(int) * (H * W + 1));

    for(i = 0; i < H * W + 1; i ++){
        masu_x[i] = 0;
        masu_y[i] = 0;
    }

    for(i = 0; i < H; i ++){
        for(j = 0; j < W; j ++){
            masu_x[masu[i][j]] = i;
            masu_y[masu[i][j]] = j;
        }
    }
    int *sum_sum = (int *) malloc(sizeof(int) * (H * W + 1));

    for(i = 1; i <= D; i ++){
        int sum = 0;

        int cur_R = i;
        int x = masu_x[cur_R];
        int y = masu_y[cur_R];

        sum_sum[i] = 0;

        for(j = i + D; j <= H * W; j += D){
            int x2 = masu_x[j];
            int y2 = masu_y[j];

            sum += abs(x2 - x) + abs(y2 - y);

            x = x2;
            y = y2;
            sum_sum[j] = sum;
        }
    }

    int Q;

    scanf("%d", &Q);

    int *L = (int *) malloc(sizeof(int) * Q);
    int *R = (int *) malloc(sizeof(int) * Q);
    int *sum = (int *) malloc(sizeof(int) * Q);

    for(i = 0; i < Q; i ++){
        scanf("%d%d", L + i, R + i);

        sum[i] = sum_sum[R[i]] - sum_sum[L[i]];
    }
    for(i = 0; i < Q; i ++){
        printf("%d\n", sum[i]);
    }

    return 0;
}