#include <stdio.h>

#define REP(i,n,m) for(i=n;i<m;i++)
#define rep(i,n) REP(i,0,n)

int n,m;
double t[100][100],dp[100][100];

double solve(int rem,int idx){
	int i;
	double tmp,res = 1;

	if(rem == 0) return 1;
	if(dp[rem][idx] != -1) return dp[rem][idx];

	rep(i,n){
		tmp = t[idx][i] * solve(rem-1,i);
		if(res < tmp) res = tmp;
	}

	return dp[rem][idx] = res;
}

int main(void){
	int i,j;
	double tmp,max;

	while(scanf("%d%d",&n,&m) && (n||m)){
		rep(i,100) rep(j,100) dp[i][j] = -1;
		rep(i,n) rep(j,n) scanf("%lf",&t[i][j]);

		max = 0;
		rep(i,n){
			tmp = solve(m-1,i);
			if(max < tmp) max = tmp;
		}
		printf("%.2lf\n",max);
	}

	return 0;
}