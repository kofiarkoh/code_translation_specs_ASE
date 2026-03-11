#include<stdio.h>
int main()
{
    int n,m,i,k=0;
    scanf("%d",&n);
    int a[n+1];
    for(i=0;i<=n-1;i++)
        scanf("%d",&a[i]);
    a[n]=-1;
    scanf("%d",&m);
    int b[m+1];
    for(i=0;i<=m-1;i++)
        scanf("%d",&b[i]);
    b[m]=-1;
    for(i=0;i<=n||i<=m;i++)
    {
        if(b[i]>a[i])
        {
            k++;
            printf("1\n");
            break;
        }
        else if(b[i]<a[i])
        {
            k++;
            printf("0\n");
            break;
        }
    }
    if(k==0)
        printf("0\n");
    return 0;
}

