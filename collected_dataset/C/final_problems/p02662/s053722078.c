#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdio.h>
#define MOD 998244353
int main(int argc, const char * argv[]) {
    int n,s;
    scanf("%d%d",&n,&s);
    int a[n];
    for(int i=0;i<n;i++){
        scanf("%d",&a[i]);
    }
    long long dp[n+1][s+1];
    for(int i=0;i<=n;i++){
        for(int j=0;j<=s;j++){
            dp[i][j]=0;
        }
    }
    dp[0][0]=1;
    for(int i=0;i<n;i++){
        for(int j=0;j<=s;j++){
            dp[i+1][j]+=dp[i][j]*2;//Tの部分集合Uに選ばない場合とTに選ばない場合
            if(a[i]+j<=s&&dp[i][j]>0){
                dp[i+1][j+a[i]]+=(dp[i][j]);//Uに選ぶ場合
                dp[i+1][j]%=MOD;
            }
        }
    }
    printf("%lld",dp[n][s]);
    return 0;
}
