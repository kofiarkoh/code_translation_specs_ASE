#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define ll long long
#define ull unsigned long long
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
ll mumll(ll p,ll q,ll m){ull x=0,pp=p;for(;q;q/=2)q&1?x+=pp,x>=m?x-=m:0:0,pp*=2,pp>=m?pp-=m:0;return(ll)x;}
ll pomll(ll a,ll n,ll m){ll x=1;for(;n;n/=2)n&1?x=mumll(x,a,m):0,a=mumll(a,a,m);return x;}
//#define MOD 998244353
#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

//非再帰fft
void fft(ll*a,int n,ll g,int p){
	for(int block=n;block>=2;block/=2){
		ll gi=1;
		for(int i=0;i<block/2;i++){
			for(int start=0;start<n;start+=block){
				//a[start+i]とa[start+i+block/2]を合体
				int temp=a[start+i]-a[start+i+block/2];
				a[start+i]+=a[start+i+block/2];
				if(a[start+i]>=p)a[start+i]-=p;
				a[start+i+block/2]=(temp+p)*gi%p;
			}
			gi=gi*g%p;
		}
        g=g*g%p;
	}

	//bit reverse
	int i=0;
	rep(j,1,n-1){
		for(int k=n>>1;k>(i^=k);k>>=1);//rev(i)に2進の+1をやる気持ち
		if(j<i){int t=a[j];a[j]=a[i];a[i]=t;}
    }
}

ll*comb(ll*a,int n,ll*b,int m,ll p,ll g){
	//0詰め
	int t=1;
	while(n+m-1>t)t*=2;
	if(t>(1<<26)){puts("error");exit(0);}
	ll*c=(ll*)calloc(t,sizeof(ll));
	memcpy(c,a,n*sizeof(ll));
	ll*d=(ll*)calloc(t,sizeof(ll));
	memcpy(d,b,m*sizeof(ll));
	//fft
	g=pom(g,(p-1)/t,p);
	fft(c,t,g,p);
	fft(d,t,g,p);
	//掛け算
	rep(i,0,t)c[i]=c[i]*d[i]%p;
	free(d);
	//復元
	fft(c,t,invp(g,p),p);
	ll coef=invp(t,p);
	rep(i,0,t)c[i]=c[i]*coef%p;
	return c;
}

ll exgcd(ll p,ll q,ll*x,ll*y){
	ll tx,ty,t,x2=0,y2=1;
	*x=1;*y=0;
	while(q){
		tx=*x-p/q*x2;*x=x2;x2=tx;
		ty=*y-p/q*y2;*y=y2;y2=ty;
		t=p%q;p=q;q=t;
	}
	return p;
}

ll crt(ll a,ll p,ll b,ll q){
	a%=p;if(a<0)a+=p;
	b%=q;if(b<0)b+=q;
	if(a>b){
		ll t;
		t=a;a=b;b=t;
		t=p;p=q;q=t;
	}
	ll x,y,m,d;
	d=exgcd(p,q,&x,&y);//px+qy=gcd(p,q)
	if((a-b)%d)return-1;
	m=p/d*q;
	//px+qy=gcd(p,q) mod lcm(p,q)
	if(x<0)x+=m/p;
	if(m<11LL<<28)return(a+(b-a)/d*p%m*x)%m;
	return(a+(ull)mumll(mumll((b-a)%q/d,p,m),x,m))%m;
}

#define FFTCNT 2
ll*comb2(ll*a,int n,ll*b,int m){
	ll p[FFTCNT]={998244353,1004535809};
	ll g[FFTCNT]={3,3};
	ll *ret[FFTCNT];
	rep(i,0,FFTCNT)ret[i]=comb(a,n,b,m,p[i],g[i]);

	rep(i,0,n+m-1)ret[0][i]=crt(ret[0][i],p[0],ret[1][i],p[1]);
	rep(i,1,FFTCNT)free(ret[i]);
	return ret[0];
}
#undef FFTCNT



#define M 200003
int a[200010];
ll e1[200010];
ll e2[200010];
int main(){
	int n;
	scanf("%d",&n);
	rep(i,0,n){
		int x;
		scanf("%d",&x);
		a[x]++;
	}
	
	
	ll ans=0;
	ll g=1;
	rep(i,0,M-1){
		e1[i]=e2[i]=a[g];
		ans-=g*g%M*a[g];
		g=g*2%M;
	}
	//printf("%lld\n",ans);
	//rep(i,0,M)if(e1[i])printf("%lld %lld %lld\n",i,e1[i],pom(2,i,M));
	
	ll *p=comb2(e1,M,e2,M);
	
	g=1;
	rep(i,0,2*(M-1)){
		ans+=g*p[i];
		g=g*2%M;
		//if(p[i])printf("%lld %lld %lld\n",i,p[i],pom(2,i,M));
	}
	
	printf("%lld\n",ans/2);
	
}