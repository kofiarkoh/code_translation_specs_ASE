#include<stdio.h>
#define ll long long
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
#define MOD 998244353

int main(){
	int n;
	scanf("%d",&n);
	ll ans=0;
	for(int i=0;i<n;i++){
		int x;
		scanf("%d",&x);
		ans=(ans+pom(2,n-1-i,MOD)*x%MOD*pom(x+1,i,MOD))%MOD;
	}
	printf("%d\n",ans);
}
