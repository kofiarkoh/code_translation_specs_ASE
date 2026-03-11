#include <stdio.h>
#define MOD 1000000007
int prime[1000000+10];
void makep(int n){prime[0]=prime[1]=1;for(int i=2;i*i<=n;i++)if(!prime[i])for(int j=i*i;j<=n;j+=i)prime[j]=1;}

int dp[100010];//a[i]を素数として使う場合の数
int a[100010];
int main(){
	makep(1000005);
	int n;
	scanf("%d",&n);
	for(int i=1;i<=n;i++)scanf("%d",a+i);
	dp[1]=!prime[a[1]];
	
	for(int i=2;i<=n;i++)if(!prime[a[i]]){
		//素数
		if(a[i-1]<a[i])dp[i]+=dp[i-1];
		if(a[i-2]<a[i])dp[i]+=dp[i-2];
		dp[i]%=MOD;
	}
	printf("%d\n",(dp[n]+dp[n-1])%MOD);
}
