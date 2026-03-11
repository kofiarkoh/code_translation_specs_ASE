#include <stdio.h>
#include <stdlib.h>

int main()
{
    int n,a,b,sum=0,n1,n2,i;
    scanf("%d%d%d",&n,&a,&b);
    scanf("%d",&n1);
    i=n-1;
    while(i--)
    {
        scanf("%d",&n2);
        if(a*(n2-n1)>b) sum=sum+b;
        else sum=sum+a*(n2-n1);
        n1=n2;
    }
    printf("%d",sum);
    return 0;
}