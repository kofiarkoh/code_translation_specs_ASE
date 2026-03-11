#include <stdio.h>
 
#define N 100
 
int G[N][N], S[N], C[N];
 
void numchk(int a, int k){
    int i;
 
    C[a] = k;
 
    for (i = 1; i <= S[a]; i++){
        if (C[G[a][i]] > k){
            numchk(G[a][i], k + 1);
        }
    }
}
 
int main(void){
    int i,j;
    int n,l;
 
    scanf("%d", &n);
 
    for (i = 1; i <= n; i++){
        C[i] = N * N;
        for (j = 1; j <= n; j++){
            G[i][j] = 0;
        }
    }
 
    for (i = 1; i <= n; i++){
        scanf("%d", &l);
        scanf("%d", &S[l]);
        for (j = 1; j <= S[l]; j++){
            scanf("%d", &G[l][j]);
        }
    }
 
    numchk(1, 0);
 
    for (i = 1; i <= n; i++){
        if (C[i] == N * N){
            C[i] = -1;
        }
        printf("%d %d\n", i, C[i]);
    }
 
    return 0;
}
