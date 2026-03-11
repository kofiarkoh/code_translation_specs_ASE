#include<stdio.h>
int main()
{
    int n,m,x[100001],a,b,i;
    scanf("%d%d",&n,&m);
    for(i=1;i<=n;i++)
    {
        scanf("%d%d",&a,&b);
        x[a]++;
        x[b]++;
    }
    for(i=1;i<=n;i++)
    {
        if(x[i]%2!=0)
        {
            printf("NO\n");
            return 0;
        }
    }
    printf("YES\n");
    return 0;
}
