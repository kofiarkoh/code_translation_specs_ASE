#include <stdio.h>

int isprime(int n) {
    int i;
    for(int i = 2; i * i <= n; i++) {
        if(n % i == 0) return 0;
    }
    return 1;
}

int main(){
    int k,suu,an1,an2;
    while(1){
        scanf("%d",&suu);
        if(suu == 0)  break;
        for(k=suu;k>0;k--){
            if(isprime(k-2)==1 && isprime(k)==1){
                an1=k-2;
                an2=k;
                break;
            }
        }
        printf("%d %d\n",an1,an2);
    }
}