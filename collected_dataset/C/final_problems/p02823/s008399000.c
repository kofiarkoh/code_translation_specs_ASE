#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main(void){
    long long int a,b,c,n;
    scanf("%lld %lld %lld",&n,&a,&b);
    if(a >= b){
        c = a; a = b; b = c;
    }
    if(a % 2 == 0){
        if(b % 2 == 0){
            printf("%lld\n",((a + b) / 2) - a);
        }
        else{
            if(a - 1 >= n - b){
                printf("%lld",n - a);
            }
            else{
                printf("%lld",b - 1);
            }
        }
    }
    if(a % 2 == 1){
        if(b % 2 == 1){
            printf("%lld",((a + b) / 2) - a);
        }
        else{
            if(a - 1 >= n - b){
                printf("%lld",n - a);
            }
            else{
                printf("%lld",b - 1);
            }
        }
    }
    return 0;
}