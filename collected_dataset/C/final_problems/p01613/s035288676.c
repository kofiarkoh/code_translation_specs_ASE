#include<stdio.h>
#define For(i,s,n,c) for (i=(s); i<n; i+=(c))
int abs(a)
        int a;
{
    return a>=0? a:-a;
}
int min(a,b)
        int a;
        int b;
{
    return a<b? a:b;
}
int main()
{
    int i,j,n,z[4];
    int x1,y1,x2,y2,sum,ans;
    ans=1000;
    scanf("%d", &n);
    For (i,0,4,1) scanf("%d",&z[i]);
    For (i,0,4,2) fprintf(stderr, "%d,", z[i]);
    For (i,1,n+1,1) {
        sum=0;
        For (j,0,4,2) {
            x1=z[j]/i+1;
            x2=z[j+1]/i+1;
            y1=z[j]%i;
            y2=z[j+1]%i;
            sum+=(abs(x1-x2)+abs(y1-y2));
        }
        ans=min(ans, sum);
    }
    printf("%d\n",ans);
}