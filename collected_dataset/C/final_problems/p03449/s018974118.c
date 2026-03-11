#include<stdio.h>
int main()
{
    int n,a[2][100],i,j,sum,max=0;
    scanf("%d",&n);
    for(i=0;i<2;i++)
        for(j=0;j<n;j++)
        scanf("%d",&a[i][j]);
    for(i=0;i<n;i++)
    {
        sum=0;
        for(j=0;j<=i;j++)
            sum+=a[0][j];
        for(j=i;j<n;j++)
            sum+=a[1][j];
        if(max<sum)
            max=sum;
    }
    printf("%d\n",max);
    return 0;
}
