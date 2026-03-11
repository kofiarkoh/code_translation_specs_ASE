#include <stdio.h>

int alpha[26];

int main(void){
    long n;
    scanf("%ld\n", &n);
    for(long i=0; i<n; i++){
        int c = getchar();
        alpha[c-'a']++;
    }
    long ans = 1;
    for(long i=0; i<26; i++){
        ans = ans * (alpha[i]+1) % 1000000007;
    }
    printf("%ld\n", ans-1);
    return 0;
}