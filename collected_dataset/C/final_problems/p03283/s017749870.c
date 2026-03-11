#include<stdio.h>
int main()
{
    int n,m,z,i,j,count=0;
    scanf("%d%d%d",&n,&m,&z);
    int l[m],r[m],p[z],q[z];
    for(i=1;i<=m;i++)
    {
        scanf("%d%d",&l[i],&r[i]);
    }
    for(i=1;i<=z;i++)
    {
        scanf("%d%d",&p[i],&q[i]);
    }
    for(i=1;i<=z;i++)
    {
        for(j=1;j<=m;j++)
        {
            if(l[j]>=p[i] && r[j]<=q[i])
            {
                count++;
            }
        }
        printf("%d\n",count);
        count=0;
    }
}