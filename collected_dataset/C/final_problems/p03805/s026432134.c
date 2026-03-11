#include <stdio.h>
#define REP(i, n) for(i = 0; i < n; ++i)
#define FOR(i, a, b) for(i = a; i < b; ++i)
#define BORDER 400

int cnt = 0;

typedef struct{
    char node[8];
}list;

void search(int nownode, list visited, int N, char matrix[N][N]){
    int i;
    char finish = 1;

    REP(i, N){
        if(visited.node[i] != 1){
            finish = 0;
            break;
        }
    }

    if(finish){
        cnt++;
        return;
    }

    REP(i, N){
        if(matrix[nownode][i] && visited.node[i] != 1){
            visited.node[i] = 1;
            search(i, visited, N, matrix);
            visited.node[i] = 0;
        }
    }
    return;
}

int main(){
    int i, j;
    int a, b;
    int N = 0, M = 0;

    scanf("%d %d", &N, &M);
    char matrix[N][N];
    list visited;

    REP(i, N){
        visited.node[i] = (i == 0 ? 1 : 0);
        REP(j, N){
            matrix[i][j] = (i == j)? 1 : 0;
        }

    }

    REP(i, M){
        scanf("%d %d", &a, &b);
        matrix[a - 1][b - 1] = 1;
        matrix[b - 1][a - 1] = 1;
    }
    search(0, visited, N, matrix);
    printf("%d\n", cnt);
    return 0;
}