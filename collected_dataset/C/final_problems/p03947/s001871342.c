#include<stdio.h>

char S[100001];

int main(void){
    char c;
    long i, ans = 0;
    
    scanf("%s", S);
    
    c = S[0];
    
    for(i = 1; S[i] != '\0'; i++){
        if(c != S[i]) ans++;
        c = S[i];
    }
    
    printf("%ld\n", ans);
    
    return 0;
}