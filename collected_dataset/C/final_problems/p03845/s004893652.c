#include<stdio.h>
int main()
{
    int n,m,i,j,sum=0,c;
    scanf("%d", &n);
    int a[n];
    for(i=0;i<n;i++)
    {
        scanf("%d", &a[i]);
    }
    scanf("%d", &m);
    int b[m][2];
    for(i=0;i<m;i++)
    {
        scanf("%d %d", &b[i][0],&b[i][1]);
    }
    for(i=0;i<m;i++)
    {
        sum=0;
        for(j=0;j+1<b[i][0];j++)
        {
            sum=sum+a[j];
        }
        sum=sum+b[i][1];
        for(j=b[i][0];j<n;j++)
        {
            sum=sum+a[j];
        }
        printf("%d\n", sum);
    }
    return 0;
}
