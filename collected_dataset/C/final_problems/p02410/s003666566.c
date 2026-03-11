#include <stdio.h>
int main(void)
{
    int n,m,a[100][100],b[100],c[100]={0},i,j;
    
    scanf("%d %d",&n,&m);
    
    for(i=1;i<=n;i++)
    {
        for(j=1;j<=m;j++)
        {
            scanf("%d",&a[i-1][j-1]);
        }
    }
    
    for(i=1;i<=m;i++)
    {
        scanf("%d",&b[i-1]);
    }
    
    for(i=1;i<=n;i++)
    {
        for(j=1;j<=m;j++)
        {
            c[i-1]=c[i-1]+a[i-1][j-1]*b[j-1];
        }
        printf("%d\n",c[i-1]);
    }
    
    return 0;
}

