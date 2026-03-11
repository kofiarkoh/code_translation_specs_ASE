#include<stdio.h>
#include<stdlib.h>

int main()
{
        int n;
        int m;
        long long x[100000];
        long long y[100000];
        const long long modnum=1000000007;
        int i;
        int j;
        int k;
        int l;
        long long width;
        long long height;
        long long area=0;

        scanf("%d %d",&n,&m);
        for(i=0;i<n;i++) scanf("%lld",&x[i]);
        for(i=0;i<m;i++) scanf("%lld",&y[i]);

        height=0;
        for(k=0;k<m;k++)
        {
                        for(l=k+1;l<m;l++) height+=y[l]-y[k];
        }

        for(i=0;i<n;i++)
        {
                for(j=i+1;j<n;j++)
                {
                        width=x[j]-x[i];
                        area=area+height*width;
                        area=area%modnum;
                }
        }
        printf("%lld\n",area);
        return 0;
}
