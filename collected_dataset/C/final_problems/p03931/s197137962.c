#include<stdio.h>

int dp[100][256][101];
long long mod,p[101];

int main(){
	int n,k,i,a[255],j,ii;
	long long sum=0;
	scanf("%d %d",&n,&k);
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
	}
	mod=1000000007;
	p[0]=1;
	for(i=1;i<=n;i++){
		p[i]=i*p[i-1];
		p[i]%=mod;
	}
	dp[0][0][0]=1;
	dp[0][a[0]][1]=1;
	for(i=1;i<n;i++){
		for(j=0;j<256;j++){
			for(ii=0;ii<i+1;ii++){
				dp[i][j][ii+1]+=dp[i-1][j^a[i]][ii];
				dp[i][j][ii+1]%=mod;
				dp[i][j][ii]+=dp[i-1][j][ii];
				dp[i][j][ii]%=mod;
			}
		}
	}
	for(i=1;i<=n;i++){
		sum+=dp[n-1][k][i]*p[i];
		sum%=mod;
	}
	printf("%lld\n",sum);
	return 0;
}
