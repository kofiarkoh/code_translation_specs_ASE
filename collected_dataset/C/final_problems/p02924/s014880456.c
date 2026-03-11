
#include<stdio.h>
int main()
{
    int n,num,i,sum=0;
    scanf("%d",&n);
    num=n-1;
    for(i=1;i<=num;i++)
    {
        sum=sum+i;
    }
    printf("%d\n",sum);

}
