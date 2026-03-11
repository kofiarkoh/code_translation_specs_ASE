#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

int main(void){

    char S[100001];

    scanf("%s", S);

    int len = strlen(S);

    int ans = 0;

    for(int i = 1; i < len; i++){
        if(S[i] != S[i-1]){
            ans++;
        }
    }

    printf("%d\n", ans);

    return 0;
}