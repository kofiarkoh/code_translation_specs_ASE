#include<stdio.h>
#define ll long long
ll calcsub(ll a,ll b,ll m,ll n){
	//x=0,x=n,y=(-ax+b)/m,y=0で囲まれる領域内の格子点
	//ただし、軸上は含まず他の境界は含む
	ll ans=0;
	ll r=(-a*n+b)/m;
	ans=r*n;
	b-=r*m;
	ll d=a/m;
	ans+=n*(n-1)/2*d;
	a-=d*m;
	b-=d*m*n;
	if(a==0)return ans;
	return ans+calcsub(m,b,a,b/m);
}

ll calc(ll a,ll b,ll m,ll n){
	//x=0,x=n,y=(ax+b)/m,y=0で囲まれる領域内の格子点
	//ただし、x=n,y=0上は含まず、x=0,y=(ax+b)/m上は含む
	return calcsub(a,a*n+b,m,n);
}

int main(){
	int q;
	scanf("%d",&q);
	while(q--){
		int a,b,m,n;
		scanf("%d%d%d%d",&n,&m,&a,&b);
		printf("%lld\n",calc(a,b,m,n));
	}
}