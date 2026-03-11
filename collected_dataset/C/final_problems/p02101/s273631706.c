#include<stdio.h>
#define min(p,q)((p)<(q)?(p):(q))
#define max(p,q)((p)>(q)?(p):(q))

int n,p,dp[110][110],x[110],y[110];
int main(){
	scanf("%d%d",&n,&p);
	for(int i=0;i<n;i++)scanf("%d%d",x+i,y+i);
	for(int k=n-1;k>=0;k--)for(int m=0;m<=n-k;m++){
		int ans=1e9,s=0,pre=0;
		for(int i=k;i<=min(n,k+m);i++){
			if(m+k<n||i==n)ans=min(ans,dp[i+1][m-(i-k)]+s);
			s+=pre=max(0,y[i]-max(0,x[i]-pre));
		}
		dp[k][m]=ans;
	}
	while(dp[0][n]>p)n--;
	printf("%d\n",n);
}
