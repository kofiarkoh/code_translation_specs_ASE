#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <stdio.h>

int main()
{   int a,b,n,k,i,j;
    scanf("%d",&k);
    if(k%2==0){a=k/2;
    a=a*a;
    
    printf("%d",a);}
    else{
        a=k/2;
    a=a*(a+1);
    printf("%d",a);
    }return 0;
}

