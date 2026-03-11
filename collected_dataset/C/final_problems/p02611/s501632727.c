#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

//Berlekamp-Massey algorithm(mod p)
//n個の項を引いてn次以下の漸化式を返す(n/2次以上なら項数不足)
//例えばフィボナッチ数列を引くと{1,-1,-1}がかえる
ll*BM(ll*s,int n,int p){
	ll *c=(ll*)calloc(n*2,sizeof(ll));
	ll *cl=(ll*)calloc(n*2,sizeof(ll));
	ll *t=(ll*)calloc(n*2,sizeof(ll));
	ll L=0,l=-1,d,dl=1;
	c[0]=cl[0]=1;

	rep(i,0,n){
		d=0;
		rep(j,0,L+1)d=(d+c[j]*s[i-j])%p;
		if(d!=0){
			rep(j,0,n)t[j]=c[j];
			rep(j,0,n)c[j+i-l]=((c[j+i-l]-d*invp(dl,p)%p*cl[j])%p+p)%p;
			if(2*L<=i){
				l=i;
				rep(j,0,n)cl[j]=t[j];
				dl=d;
				L=i+1-L;
			}
		}
	}
	free(cl);
	free(t);
	return c;
}

//きたまさ法（線形漸化式の第n項(0-based)をmodmで高速に求める）
int kitamasa(ll*coe,ll*A,int size,ll n,int m){
	/*
	例：フィボナッチ数列
	size=2;//特性方程式の次数
	coe[]={1,1};//特性方程式の最高次の係数を-1にして、消したもの
	A[]={0,1};//初期値
	*/

	ll*temp=(ll*)calloc(2*size,sizeof(ll));
	ll*s=(ll*)calloc(size,sizeof(ll));
	ll*ans=(ll*)calloc(size,sizeof(ll));

	s[1]=1;
	ans[0]=1;

	while(n){
		if(n%2){
			rep(i,0,2*size)temp[i]=0;
			rep(i,0,size)rep(j,0,size)temp[i+j]=(temp[i+j]+ans[i]*s[j])%m;
			for(ll i=2*size-2;i>=size;i--)rep(j,0,size)temp[i-size+j]=(temp[i-size+j]+temp[i]*coe[j])%m;
			rep(i,0,size)ans[i]=temp[i];
		}
		rep(i,0,2*size)temp[i]=0;
		rep(i,0,size)rep(j,0,size)temp[i+j]=(temp[i+j]+s[i]*s[j])%m;
		for(ll i=2*size-2;i>=size;i--)rep(j,0,size)temp[i-size+j]=(temp[i-size+j]+temp[i]*coe[j])%m;
		rep(i,0,size)s[i]=temp[i];
		n/=2;
	}
	ll ss=0;
	rep(i,0,size)ss=(ss+ans[i]*A[i])%m;
	free(temp);
	free(s);
	free(ans);
	return ss;
}


ll fact[100010];
ll invfact[100010];
void makefact(int n,ll m){
	fact[0]=1;
	rep(i,0,n)fact[i+1]=fact[i]*(i+1)%m;
	invfact[n]=invp(fact[n],m);
	for(int i=n;i>0;i--)invfact[i-1]=invfact[i]*i%m;
}
ll choose(int n,int r,int m){return n<0||r<0||n-r<0?0:fact[n]*invfact[r]%m*invfact[n-r]%m;}

ll f(int n){
	ll ans=0;
	rep(i,5,n+1){
		ans+=choose(i+4,9,MOD)*choose((n-i)/2+5,5,MOD);
		ans%=MOD;
	}
	return ans;
}

#define N 100
ll a[N+10];
ll coe[N+10];
int main(){
	makefact(N+10,MOD);
	rep(i,0,N)a[i]=f(i);
	ll*p=BM(a,N,MOD);
	int deg=0;
	rep(i,0,N)if(p[i])deg=i;
	rep(i,0,deg)coe[i]=(MOD-p[deg-i])%MOD;

	int t;
	scanf("%d",&t);
	while(t--){
		int n;
		scanf("%d",&n);
		printf("%d\n",kitamasa(coe,a,deg,n,MOD));
	}
}

