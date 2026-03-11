#include <stdio.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define MOD 1000000007

char s[3010];
ll dp[3010][3010];//dp[i][j]=(i番目まで決めて、i番目より大きいものがj個残っている)

int main(){
	ll n;
	scanf("%lld %s",&n,s);
	rep(j,0,n)dp[1][j]=1;
	rep(i,1,n){
		ll temp[3010]={};
		if(s[i-1]=='<'){
			//大きいものから使うので、jは減る
			//配るDP
			//rep(j,0,n-i+1)rep(k,0,j)dp[i+1][k]=(dp[i+1][k]+dp[i][j])%MOD;
			rep(j,0,n-i+1)temp[0]+=dp[i][j],temp[j]-=dp[i][j];
		}else{
			//小さいものから使うので、jは増える
			//配るDP
			//rep(j,0,n-i+1)rep(k,j,n-i+1)dp[i+1][k]=(dp[i+1][k]+dp[i][j])%MOD;
			rep(j,0,n-i+1)temp[j]+=dp[i][j],temp[n-i+1]-=dp[i][j];
		}
		dp[i+1][0]=temp[0];
		rep(j,0,n-i+1)dp[i+1][j+1]=(dp[i+1][j]+temp[j+1])%MOD;
	}
	printf("%lld\n",(dp[n][0]+MOD)%MOD);
	return 0;
}