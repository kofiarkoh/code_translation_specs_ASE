#include <stdio.h>
#include <stdlib.h>
#define max(p,q)((p)>(q)?(p):(q))
#define bit(n,m)(((n)>>(m))&1)

int dp[10010];
int main(){
	int n,m;
	scanf("%d%d",&n,&m);
	while(n--){
		int v,w,c;
		scanf("%d%d%d",&v,&w,&c);
		int k;
		for(k=1;2*k-1<=c;k*=2){
			for(int i=m-w*k;i>=0;i--)dp[i+w*k]=max(dp[i+w*k],dp[i]+v*k);
		}
		k=c-k+1;
		for(int i=m-w*k;i>=0;i--)dp[i+w*k]=max(dp[i+w*k],dp[i]+v*k);
	}
	printf("%d\n",dp[m]);
}
