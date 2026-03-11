#include<stdio.h>

double power(long long int x,long long int n){
    long long int M=1000000007,tmp;
    
    if(n==0) return 1;
    tmp=power(x*x%M,n/2);
    if(n%2==1){
        tmp=tmp*x%M;
    }    
    return tmp;
}

int main(void){
long long int m,n,x;
scanf("%lld %lld",&m,&n);
x=power(m,n);
printf("%lld\n",x);

return(0);
}


