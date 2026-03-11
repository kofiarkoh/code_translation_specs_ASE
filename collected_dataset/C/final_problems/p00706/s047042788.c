#include <stdio.h>

int dir_x[] = {1, -1};
int dir_y[] = {1, -1};

int main(void)
{
    while(1){
        int N, W, H, S, T;
        int field[101][101] = {{0}};

        scanf("%d", &N);
        if(N == 0)
            break;
        scanf("%d %d", &W, &H);

        int i;
        int tx, ty;
        for(i = 0; i < N; i++){
            scanf("%d %d", &tx, &ty);
            field[ty][tx] = 1;
        }
        scanf("%d %d", &S, &T);

        int max = 0;
        int sx, sy;
        for(sx = 1; sx <= W; sx++){
            for(sy = 1; sy <= H; sy++){

                int j;
                for(j = 0; j < 2; j++){
                    int k;
                    for(k = 0; k < 2; k++){
                        tx = sx;
                        ty = sy;
                        if(tx + dir_x[j] * S <= W && tx + dir_x[j] * S > 0 && ty + dir_y[k] * T <= W && ty + dir_y[k] * T > 0){
                            int temp = 0;
                            int g_x = tx + dir_x[j] * S;
                            int g_y = ty + dir_y[k] * T;
                            for(tx = sx; tx != g_x; tx += dir_x[j])
                                for(ty = sy; ty != g_y; ty += dir_y[k]){
                                    if(field[ty][tx])
                                        temp++;
                                }
                            if(temp > max)
                                max = temp;
                        }
                    }
                }
            }
        }

        printf("%d\n", max);
    }

    return 0;
}