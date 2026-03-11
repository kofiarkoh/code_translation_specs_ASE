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

uint min(uint a, uint b){
    return a < b? a : b;
}

void TF_print(char boolean, char T[], char F[]){
    if(boolean){
        printf("%s\n", T);
    }else{
        printf("%s\n", F);
    }
}

int main(){
    char S[200001];
    scanf("%s", S);
    uint len = strlen(S);
    uint i,s = 0, save = 0, t = 0, count = 0;

    REP(i, len){
        if(S[i] == 'S'){
            if(s == 0){
                count += min(save, t) * 2;
                t = 0;
            }
            s++;
        }else if(S[i] == 'T'){
            if(t == 0){
                save = s;
                s = 0;
            }
            t++;
        }
    }
    count += min(save, t) * 2;
    printf("%u\n", len - count);
    return 0;
}