#include<stdio.h>

int fibo(int n){
    int f,i;
    int fib[50];
    for(i=0; i<=n; i++){
        if(i==0 || i==1)fib[i] = 1;
        else fib[i] = fib[i-2] + fib[i-1];
    }
    if(n==0 || n==1) return fib[n];
    else {
        f = fib[n-1]+fib[n-2];
        return f;
    }
}
int main(){
    int n;
    scanf("%d", &n);
    printf("%d\n",fibo(n));
    return 0;
}
