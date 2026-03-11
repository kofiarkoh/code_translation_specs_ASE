#define UNDERSTAND 1000000007

#include <stdio.h>

int main(void)
{
    
    long int n,m;
    long int temp=1;
    long int change;
    
    scanf("%ld %ld",&n,&m);
    
    if(n < m){
        change = m;
        m = n;
        n = change;
    }
    if((n-m==1)||(m-n==1)||(m==n)){
        
        int count = m;
        
        while(count>1){
            temp = temp * count--;
            temp = temp % UNDERSTAND;
        }
        
        temp = temp * temp;
        temp = temp % UNDERSTAND;
        
        if(n > m){
            temp = temp * n;
            temp = temp% UNDERSTAND;
        }
        
        if(n == m)printf("%ld\n",(temp*2) % UNDERSTAND);
        else printf("%ld\n",temp % UNDERSTAND);
    }
    else printf("0\n");
    
    return 0;
}