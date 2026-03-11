#include <stdio.h>
#include <string.h>

#define REP(i,n,m) for(i=n;i<m;i++)
#define rep(i,n) REP(i,0,n)

int sq[17];
int dp[301][18];

int solve(int rem,int idx){
	int i,res=0;

	if(rem == 0) return 1;
	if(rem < 0) return 0;
	if(dp[rem][idx] != -1) return dp[rem][idx];

	REP(i,idx,17){
		res += solve(rem - sq[i], i);
	}
	return dp[rem][idx] = res;
}

int main(void){
	int i,n;

	REP(i,1,18) sq[i-1] = i * i;
	memset(dp,-1,sizeof(dp));

	while(scanf("%d",&n) && n){
		printf("%d\n",solve(n,0));
	}

	return 0;
}