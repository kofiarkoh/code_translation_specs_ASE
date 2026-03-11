#include <stdio.h>


int wall[100];
int out[51];

int main(void)
{
    int N, M;
    int a, L;
    int i, j;
    int start, end, pos;
    int len;
    int flag;
    
    scanf("%d%d", &N, &M);
    
    for (i = 0; i < M; i++){
        scanf("%d%d", &a, &L);
        
        for (j = a; j < a + L; j++){
            wall[j % N] = 1;
        }
    }

#if 0
for (i = 0; i < N; i++){
    printf("%d", wall[i]);
}
printf("\n");
#endif

    for (start = 0; start < N; start++){
        if (wall[start] == 0){
            break;
        }
    }
    if (start == N){
        printf("%d 1\n", N);
        goto END;
    }
//printf("start=%d\n", start);
    pos = start;
    len = 0;
    flag = 0;
    do {
        if (len == 0){
            if (wall[pos] == 1){
                len = 1;
//printf("+");
            }
        }
        else {
            if (wall[pos] == 1){
                len++;
            }
            else {
                out[len]++;
//printf("len=%d\n", len);
                len = 0;
                flag = 1;
            }
        }
        pos++;
        pos %= N;
    } while (pos != start);
    if (len > 0){
        out[len]++;
        flag = 1;
    }
    
    if (flag == 0){
        printf("0 0\n");
    }
    else {
        for (i = 50; i >= 0; i--){
            if (out[i] > 0){
                printf("%d %d\n", i, out[i]);
            }
        }
    }
    
END:;
    return (0);
}

