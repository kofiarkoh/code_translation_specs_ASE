#include<stdio.h>
int main()
{
    int n,m,n1,m1,sum;
    scanf("%d%d%d%d",&n,&m,&n1,&m1);
    if(n-m>=0)
    {
        sum=m*n1+(n-m)*m1;
    }
    else
        sum=n*n1;
        printf("%d\n",sum);
    return 0;
}
