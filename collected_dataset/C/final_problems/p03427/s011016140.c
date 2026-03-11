#include <stdio.h>
#include <math.h>
int main()
{   long long n;
    scanf("%lld",&n);
    long long cpy=n;
    int sum=0;
    int dig=0;
    while(cpy>0){
        sum+=cpy%10;
        cpy/=10;
        dig++;
    }
    int head=n/pow(10,(dig-1));
    if((sum-head)==9*(dig-1)){
        printf("%d",(dig-1)*9+head);
    }else{
        printf("%d",(dig-1)*9+head-1);
    }
    return 0;
}
