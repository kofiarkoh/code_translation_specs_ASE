#include<stdio.h>
int a[111],dp[111];
int main()
{
    int m,n;
    scanf("%d%d",&m,&n);
    for(int i=1;i<=m;i++)scanf("%d",&a[i]);
    long long ans=0;
    for(int i=1;i<=m;i++)
    {
        if(a[i]==n) ans=(ans+i*(m-i+1)%998244353)%998244353;
        else if(n>a[i])ans=(ans+dp[n-a[i]]*(m-i+1)%998244353)%998244353;
        for(int j=n;j>=a[i];j--) dp[j]=(dp[j]+dp[j-a[i]])%998244353;
        dp[a[i]]=(dp[a[i]]+i)%998244353;
    }
    printf("%lld",ans);
}
