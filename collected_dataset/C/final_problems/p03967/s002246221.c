#include <stdio.h>
#include <string.h>
#define FOR(i, a, b) for(i = a; i < b; ++i)
#define REP(i, n) FOR(i, 0, n)

#define N 100000

int result(char atc, char topc){
    if(atc == 'p' && topc == 'g'){
        return 1;
    }else if(atc == 'g' && topc == 'p'){
        return -1;
    }else{
        return 0;
    }
}

int max(int a, int b){
    return a > b? a : b;
}

int rock_paper(unsigned int n, unsigned int p, unsigned int len, int dp[len + 1][len + 1], char S[N]){
    if(dp[n][p] != -N){
        return dp[n][p];
    }
    if(n == len){
        return dp[n][p] = 0;
    }

    int rock = rock_paper(n + 1, p + 1, len, dp, S) + result('g', S[n]);
    if(p == 0){
        return dp[n][p] = rock;
    }else{
        int paper = rock_paper(n + 1, p - 1, len, dp, S) + result('p', S[n]);
        return dp[n][p] = max(rock, paper);
    }
}

int main(){
    char S[N];
    scanf("%s", S);
    unsigned int len = strlen(S);
    int dp[len + 1][len + 1];
    unsigned int i, j;

    if(len == 1){
        if(S[0] == 'p'){
            printf("-1\n");
        }else{
            printf("0\n");
        }
    }
    else{
        REP(i, len + 1){
            REP(j, len + 1){
                dp[i][j] = -N;
            }
        }
        printf("%d\n", rock_paper(0, 0, len, dp, S));
        /*
        REP(i, len + 1){
            REP(j, len + 1){
                printf("%d ", dp[i][j]);
            }
            printf("\n");
        }
        */

    }
    return 0;
}

/*
nターン目以降を最適化した時の最大化された点数：dp[n][p]
n:何ターン目か, p:パーを出せる回数の上限
*/