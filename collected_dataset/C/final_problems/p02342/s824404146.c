#include <stdio.h>
#define MOD 1000000007

int dp[1010][1010];
//dp[n][k]=nをk個以下の整数の和で表す場合の数
//        =nをk以下の整数の和で表す場合の数
int main(){
	int n,m;
	scanf("%d%d",&n,&m);
	for(int j=0;j<=m;j++)dp[0][j]=1;
	for(int i=1;i<=n;i++)for(int j=1;j<=m;j++)dp[i][j]=(dp[i][j-1]+(i-j>=0?dp[i-j][j]:0))%MOD;
	printf("%d\n",(dp[n][m]-dp[n][m-1]+MOD)%MOD);
}
