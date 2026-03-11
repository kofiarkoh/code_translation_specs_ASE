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
    int i,j,n,a,b,c,d;
    int x1,y1,x2,y2,sum,tmp,ans;
    ans=1000;
    scanf("%d%d%d%d%d",&n,&a,&b,&c,&d);
    fprintf(stderr, "%d", d);
    For (i,1,n+1,1) {
        sum=0;
        x1=a/i+1;
        x2=b/i+1;
        y1=a%i;
        y2=b%i;
        sum=(abs(x1-x2)+abs(y1-y2));
        x1=c/i+1;
        x2=d/i+1;
        y1=c%i;
        y2=d%i;
        sum+=(abs(x1-x2)+abs(y1-y2));
        ans=min(ans, sum);
    }
    printf("%d\n",ans);
}