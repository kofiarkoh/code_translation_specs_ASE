#include<stdio.h>
int main()
{
    int n,t[1000],m,p,x,i,j,sum,temp;
    scanf("%d",&n);
    for(i=1;i<=n;i++)
    {
        scanf("%d",&t[i]);
    }
    scanf("%d",&m);
    for(i=1;i<=m;i++)
    {
        sum=0;
        scanf("%d%d",&p,&x);
        temp=t[p];
        t[p]=x;
        for(j=1;j<=n;j++)
        {
            sum=sum+t[j];
        }
        printf("%d\n",sum);
        t[p]=temp;
    }
    return 0;
}
