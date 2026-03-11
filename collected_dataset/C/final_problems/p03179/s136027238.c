#include <stdio.h>
#define mod 1000000007
#define Maxn 3005
int n,dp[Maxn],pre[Maxn],suf[Maxn];
int main()
{
    scanf("%d\n",&n);
    dp[1]=pre[1]=suf[1]=1;
    for (int i=2;i<=n;i++)
    {
        if (getchar()=='<') for (int j=1;j<=i;j++) dp[j]=pre[j-1];
        else for (int j=1;j<=i;j++) dp[j]=suf[j];
        for (int j=1;j<=i;j++) pre[j]=(pre[j-1]+dp[j])%mod;
        for (int j=i;j>=1;j--) suf[j]=(suf[j+1]+dp[j])%mod;
    }
    printf("%d\n",pre[n]);
    return 0;
}