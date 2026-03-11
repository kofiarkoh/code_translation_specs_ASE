#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FOR(i, a, b) for(i = a; i < b; ++i)
#define REP(i, n) FOR(i, 0, n)
#define comp(a, b) int comp(const void *a, const void *b)
#define MOD  1000000007

typedef unsigned int uint;
typedef unsigned long ul;
typedef long long ll;
typedef unsigned long long ull;

void TF_print(char boolean, char T[], char F[]){
    if(boolean){
        printf("%s\n", T);
    }else{
        printf("%s\n", F);
    }
}

comp(a, b){
    ll diff = *(ll*)a - *(ll*)b;
    if(diff > 0){
        return 1;
    }else if(diff < 0){
        return -1;
    }else{
        return 0;
    }
}

int main(){
    ll N, b;
    ll i;
    scanf("%lld", &N);
    ll A[N], B[N], C[N], temp;
    ll A_less_B[N];// A[i] < B[n]なるiの個数
    ll C_big_B[N]; // B[n] < C[i]なるiの個数

    REP(b, 3){
        REP(i, N){
            scanf("%lld", &temp);
            if(b == 0) A[i] = temp;
            if(b == 1) B[i] = temp;
            if(b == 2) C[i] = temp;
        }

        if(b == 0) qsort(A, N, sizeof(ll), comp);
        if(b == 1) qsort(B, N, sizeof(ll), comp);
        if(b == 2) qsort(C, N, sizeof(ll), comp);
    }

    REP(b, N){
        A_less_B[b] = N;                   
        REP(i, N){
            if(A[i] >= B[b]){
                A_less_B[b] = i;
                break;
            }
        }
    }

    REP(b, N){
        C_big_B[b] = N;
        REP(i, N){
            if(C[N - 1 - i] <= B[b]){
                C_big_B[b] = i;
                break;
            }
        }
    }

    ll ANS = 0;
    REP(b, N){
        ANS += A_less_B[b] * C_big_B[b];
    }

    printf("%lld\n", ANS);
    return 0;
}

/*
3数の大小関係は真ん中を基準にすると良い. 4, 5と増えても同じではなかろうか.
*/