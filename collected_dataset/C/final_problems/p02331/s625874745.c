#include <stdio.h>
#define ll long long
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

int main(){
	int n,m;
	scanf("%d%d",&n,&m);
	printf("%lld\n",pom(m,n,MOD));
}
