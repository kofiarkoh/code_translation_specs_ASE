#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF ((1LL<<62)-(1LL<<31))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))
#define bit(n,m)(((n)>>(m))&1)
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
#define MOD 998244353
//#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

#define p 998244353

//非再帰fft
void fft(ll*a,int n,ll g){
	for(int block=n;block>=2;block/=2,g=g*g%p){
		ll gi=1;
		for(int i=0;i<block/2;i++,gi=gi*g%p){
			for(int start=0;start<n;start+=block){
				//a[start+i]とa[start+i+block/2]を合体
				ll temp=a[start+i]-a[start+i+block/2];
				a[start+i]+=a[start+i+block/2];
				if(a[start+i]>=p)a[start+i]-=p;
				a[start+i+block/2]=(temp+p)*gi%p;
			}
		}
	}
	//bit reverse
	int i=0;
	rep(j,1,n-1){
		for(int k=n>>1;k>(i^=k);k>>=1);//rev(i)に2進の+1をやる気持ち
		if(j<i){int t=a[j];a[j]=a[i];a[i]=t;}
    }
}


#define N (1<<17)
int a[100010];
ll b[100010];

ll ccc[100010];//2i人を組にする

ll ans[N];
ll temp[N];
int main(){
	
	ccc[0]=1;
	rep(i,1,100005)ccc[i]=(2*i-1)*ccc[i-1]%MOD;
	
	int n;
	scanf("%d",&n);
	rep(i,0,2*n){
		int t;
		scanf("%d",&t);
		a[t]++;
	}
	rep(i,0,100005)b[a[i]]++;
	
	//Π(x-i(i-1)/2)^b[i]
	ans[0]=1;
	fft(ans,N,pom(3,(p-1)/N,MOD));

	ll deg=0;
	rep(i,1,100005)if(b[i]){
		deg+=b[i];
		memset(temp,0,sizeof(ll)*N);
		temp[0]=(MOD-(i*(i-1)/2)%MOD);
		temp[1]=1;
		fft(temp,N,pom(3,(p-1)/N,MOD));
		rep(j,0,N)ans[j]=ans[j]*pom(temp[j],b[i],MOD)%MOD;
	}
	fft(ans,N,pom(3,(p-1)-(p-1)/N,MOD));
	ll coef=invp(N,p);
	rep(i,0,N)ans[i]=ans[i]*coef%MOD;
	
	
	ll aaa=0;
	rep(i,0,deg+1){
		//違反がi個,deg-i次
		ll temp=ans[deg-i]*ccc[n-i]%MOD;
		aaa+=temp;
	}
	printf("%lld",aaa%MOD);
}