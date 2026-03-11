#include<stdio.h>
#define mod 1000000007
long long ans=0;
long long s[3000][2000]={0};

int main(){
	int n;int i,j;
	scanf("%d",&n);
    for (i=3;i<=n;i++){
     s[i][1]=1;
     for (j=2;j<=i/3;j++)
     	for (int k=i-3;k>=0;k--){
     		s[i][j]=(s[i][j]+s[k][j-1])% mod;
     		if (s[k][j-1]==0) break;
		 }
		 }	 
	long long ans=0;	 
    for (i=1;i<=n/3;i++){
    	ans=(ans+s[n][i])%mod;
	}
	printf("%lld",ans);
}
	 