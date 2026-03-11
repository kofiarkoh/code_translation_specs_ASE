#include<stdio.h>
int min(int x, int y){return x > y ? y : x;}
int main(void){
    int i, j, k;
    int h, w;
    int c[10][10];
    int a[10], tmp;

    scanf("%d%d", &h, &w);
    for(i = 0; i < 10; i++){
        for(j = 0; j < 10; j++){
            scanf("%d", &c[i][j]);
        }
    }

    for(i = 0; i < 10; i++){
        a[i] = 0;
    }

    for(i = 0; i < h; i++){
        for(j = 0; j < w; j++){
            scanf("%d", &tmp);
            if(tmp == -1){
                continue;
            }else{
                a[tmp]++;
            }
        }
    }

    for(k = 0; k < 10; k++){
        for(i = 0; i < 10; i++){
            for(j = 0; j < 10; j++){
                c[i][j] = min(c[i][j], c[i][k] + c[k][j]);
            }
        }
    }

    int ans = 0;
    for(i = 0; i < 10; i++){
        ans += a[i] * c[i][1];
    }
    printf("%d\n", ans);
    return 0;
}