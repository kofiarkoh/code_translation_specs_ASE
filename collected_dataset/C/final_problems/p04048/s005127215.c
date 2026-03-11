#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

long long getgcd(long long N,long long X);

int main(void){
    long long N,X;
    long long buf;
    long long cur;
    scanf("%lld %lld",&N,&X);
    buf=getgcd(N,X);
    printf("%lld\n",3*N-3*buf);
}

long long getgcd(long long N,long long X) {
    
    long i;
    long num;
    for (i=1;i<(long long)(sqrt(N)+1);i++){
        if((fmodl(N,i)==0)&&(fmodl(X,i)==0)){
            num=i;
        }
    }
    return num;
}