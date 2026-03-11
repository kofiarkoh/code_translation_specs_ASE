#include<stdio.h>
#define ll long long
#define max(p,q)((p)>(q)?(p):(q))
#define MOD 1000000007
#define add(x,k)(x=(x+(t)*(k))%MOD)
ll dp[60][60][6000];
//dp[i][j][k]=i番目までみて、j個未確定で、奇妙さkが確定
int main(){
	int n,kk;
	scanf("%d%d",&n,&kk);
	dp[0][0][0]=1;
	for(int i=0;i<n;i++)for(int j=0;j<=i;j++)for(int k=0;k<2000;k++){
		int t=dp[i][j][k];
		//自分同士で結ぶ
		add(dp[i+1][j][k+2*j],1);
		//両方使う
		if(j)add(dp[i+1][j-1][k+2*(j-1)],j*j);
		//片方使う
		if(j)add(dp[i+1][j][k+2*j],2*j);
		//使わない
		add(dp[i+1][j+1][k+2*(j+1)],1);
	}
	printf("%lld",dp[n][0][kk]);
}