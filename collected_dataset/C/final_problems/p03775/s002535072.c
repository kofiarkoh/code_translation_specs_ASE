#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

int digit(long int n){
    int count = 0;
    while(n != 0){
        n /= 10;
        count++;
    }
    return count;
}

int max(int a, int b){
    return a > b ? a : b;
}

int main(void){
    long int N;
    scanf("%ld" ,&N);
    long int A,B;
    int i;
    int min = INT_MAX;
    int digit_A,digit_B,f;
    for(i=1;i*i<=N;i++){
        if(N % i == 0){
            A = i;
            B = N / A;
        }
        if(A > B) break;
        digit_A = digit(A);
        digit_B = digit(B);
        f = max(digit_A,digit_B);
        if(f < min) min = f;
    }
    printf("%d\n",min);
}