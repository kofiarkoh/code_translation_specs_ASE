#include<stdio.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
#define MOD 998244353
#define invp(a,p)pom(a,p-2,p)

//*
ll fact[100010];
ll invfact[100010];
void makefact(int n,ll m){
	fact[0]=1;
	rep(i,0,n)fact[i+1]=fact[i]*(i+1)%m;
	invfact[n]=invp(fact[n],m);
	for(int i=n;i>0;i--)invfact[i-1]=invfact[i]*i%m;
}
ll choose(int n,int r,int m){return n<0||r<0||n-r<0?0:fact[n]*invfact[r]%m*invfact[n-r]%m;}
//*/

ll dp[1010];
int main(){
	ll m,n,k;
	scanf("%lld%lld%lld",&m,&n,&k);
	makefact(2010,MOD);
	dp[1]=1;
	rep(i,2,k){
		dp[i]=pom(i,n,MOD);
		rep(x,1,i)dp[i]-=choose(i,x,MOD)*dp[x]%MOD;
		dp[i]%=MOD;
	}
	ll ans=pom(m,n,MOD);
	ll coe=m%MOD;
	rep(x,1,k){
		ans-=coe*dp[x]%MOD;
		coe=(m-x)%MOD*coe%MOD*invp(x+1,MOD)%MOD;
	}
	printf("%d\n",(ans%MOD+MOD)%MOD);
}
