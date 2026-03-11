#include<stdio.h>
#include<math.h>
int main(){
    long int N;
    long int x = 1;
    long int count = 0;
    scanf("%ld",&N);
    while(x*x+x < N){
        if(N%x==0){count += ((N/x)-1);}
        x++;
    }
    printf("%ld\n",count);
}