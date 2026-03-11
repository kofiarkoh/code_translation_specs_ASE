#include<stdio.h>

int main(){
    int n;
    scanf("%d",&n);
    long long int i = 1;
    long long int s = 0;
    while(1){
        s += i;
        if(s % n == 0){
            printf("%llu\n",i);
            break;
        }
        i += 1;
    }
}