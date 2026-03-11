#include<stdio.h>
#define i64 long long int
i64 m[3001][3001];
i64 mp=998244353;
int a[3001];
i64 f(int i,int s)
{
    if((i==0)&&(s==0)) return 1;
    if(i==0) return 0;
    if(m[i][s]!=-1) return m[i][s];
    if(s-a[i]>=0)
        m[i][s]=((2*f(i-1,s))%mp+f(i-1,s-a[i]))%mp;
    else m[i][s]=(2*f(i-1,s))%mp;
    return m[i][s];
}

int main()
{
   int n,s,i,j;
   i64 b;
   scanf("%d %d",&n,&s);
   for(i=1;i<=n;i++) scanf("%d",&a[i]);
   for(i=0;i<=n;i++)
   {
       for(j=0;j<=s;j++)
        m[i][j]=-1;
   }
   b=f(n,s);
   printf("%lld",b);
    return 0;
}
