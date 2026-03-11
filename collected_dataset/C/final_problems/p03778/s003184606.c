#include <stdio.h>

int main(void)
{
    long int W,a,b,exchange;
    scanf("%ld %ld %ld",&W,&a,&b);
    if(a>b){
        exchange=a;
        a=b;
        b=exchange;
    }
    
    if(b-a<=W)printf("0\n");
    else printf("%ld\n",b-a-W);
    
    return 0;
}