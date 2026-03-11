#include<stdio.h>
int main()
{
    int i,j,k,l,n,count,c=0,a[200000],sum;
    scanf("%d%d",&n,&k);
    for(i=0;i<n;i++)
    {
        scanf("%d",&a[i]);
    }
    for(i=0;i<n;i++)
    {
        for(j=i;j<n;j++)
        {
            sum=0;
            count=0;
            for(l=i;l<=j;l++)
                {
                    sum+=a[l];
                    count++;
                }
                if(count==sum%k)
                    c++;
        }
    }
    printf("%d",c);
}