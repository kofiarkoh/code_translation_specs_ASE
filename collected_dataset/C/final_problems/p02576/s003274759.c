#include<stdio.h>

int main() {
    int i,n,x,t,N=0;
    scanf("%d%d%d",&n,&x,&t);
    if(n == x){
        N = 1;
    }else if(x == 1){
        N = n;
    }else{
        N = (n/x) + 1;
    }
    printf("%d\n", N*t);
    return 0;
}