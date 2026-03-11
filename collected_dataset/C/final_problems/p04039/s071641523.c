#include<stdio.h>
int main()
{
    long long n,k,i,j,s=0,t,m=0,p=0,q=0,e,u,v;
    scanf("%lld%lld",&n,&k);
    long long  pt[k],pl[10],po[10];
    for(i=0;i<k;i++)
    {
        scanf("%lld",&pt[i]);
    }
    for(i=0;i<=9;i++)
    {
        for(j=0;j<k;j++)
       {
           if(i==pt[j])
           {
               p=1;
               break;
           }
       }
       if(p==0)
       {
           po[q] = i;
           q++;
       }
       else
       {
           p = 0;
           continue;
       }
   }
    for(i=0;i<=q;i++)
    {
        for(j=i+1;j<q;j++)
        {
            if(po[i]>po[j])
            {
                e = po[i];
                po[i] = po[j];
                po[j] = e;
            }
        }
    }
    for(i=0;n!=0;i++)
    {
        pl[i] = n%10;
        for(j=0;j<k;j++)
        {
            u = n/10;
            if(pl[i]==pt[j] && u!=0)
            {
                pl[i] = po[0];
                break;
            }
            if(pl[i]==pt[j] && u==0)
            {
                pl[i] = pl[i] +1;
                j = -1; 
            }
        }
        n = n/10;
        m++;
    }
    for(i=m-1;i>=0;i--)
    {
        printf("%lld",pl[i]);
    }
    printf("\n");
    return 0;
}