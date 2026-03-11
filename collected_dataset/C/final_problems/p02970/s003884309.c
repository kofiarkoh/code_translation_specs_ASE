#include<stdio.h>
int main(void){
    int n,d;
    scanf("%d%d",&n,&d);

    if(n % (1+2*d) == 0){
        printf("%d\n", ( n / (1+2*d) ) );
    }else{
        printf("%d\n", ( n / (1+2*d) + 1 ) );
    }

    return 0;
}