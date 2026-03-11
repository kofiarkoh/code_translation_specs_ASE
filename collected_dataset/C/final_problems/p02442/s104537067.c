#include<stdio.h>
int main()
{
    int n,m,i,sum1=0,sum2=0;
    scanf("%d",&n);
    int a[n];
    for(i=0;i<n;i++)
    {
        scanf("%d",&a[i]);
    }
        scanf("%d",&m);
        int b[m];
    for(i=0;i<m;i++)
    {
        scanf("%d",&b[i]);
    }
        for(i=0;i<n;i++){
                sum1=sum1+a[i];}
        for(i=0;i<m;i++)
        {
                sum2=sum2+b[i];
        }
        if(sum2>sum1||sum2==sum1)
            printf("1\n");
        else
            printf("0\n");
    return 0;
}

