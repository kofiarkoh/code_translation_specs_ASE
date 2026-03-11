#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

bool g[8][8];

int dfs(int v, int N, bool visited[N]){
    bool all = true;

    for(int i = 0; i < N; i++){
        if(visited[i] == false){
            all = false;
        }
    }

    if(all){
        return 1;
    }

    int r = 0;

    for(int i = 0; i < N; i++){
        if(g[v][i] == false){
            continue;
        }
        if(visited[i]){
            continue;
        }

        visited[i] = true;
        r += dfs(i, N, visited);
        visited[i] = false;
    }

    return r;
}

int main(void){

    int N, M;

    scanf("%d%d", &N, &M);

    int a[M], b[M];

    for(int i = 0; i < M; i++){
        scanf("%d%d", &a[i], &b[i]);
        g[a[i]-1][b[i]-1] = true;
        g[b[i]-1][a[i]-1] = true;
    }

    bool visited[N];

    visited[0] = true;
    for(int i = 1; i < N; i++){
        visited[i] = false;
    }

    printf("%d\n", dfs(0, N,visited));

    return 0;
}