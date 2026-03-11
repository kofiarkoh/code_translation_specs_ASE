#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF ((1LL<<62)-(1LL<<31))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))
#define bit(n,m)((n>>m)&1)
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
#define MOD 998244353
#define invp(a,p)pom(a,p-2,p)

ll fact[4010],invfact[4010];
ll factinit(ll n){
	fact[0]=1;
	rep(i,0,n)fact[i+1]=fact[i]*(i+1)%MOD;
	invfact[n]=invp(fact[n],MOD);
	for(ll i=n;i>0;i--)invfact[i-1]=invfact[i]*i%MOD;
}
ll choose(ll n,ll r){
	if(n<0||r<0||n-r<0)return 0;
	return fact[n]*invfact[r]%MOD*invfact[n-r]%MOD;
}

ll ans[4010];
int main(){
	factinit(4005);
	ll n,k;
	scanf("%lld %lld\n",&k,&n);
	rep(x,2,k+2){
		ll t=0;
		if(x%2){
			//奇数のとき
			//はずすべきペアはx/2組ある
			ll p=x/2;
			rep(i,0,p+1){
				//p組のうちちょうどi組が(0,0)だとする。その組の選び方がchoose(p,i)
				//残りの(p-i)組に先に1個わりあてると、残りの(n-p+i)個を(k-2*p+p-i)組に振り分けるので
				//choose(k+n-2*p-1,n-p+i)通り
				//(p-i)組はどっちに割り振るか各2通りあるので2^(p-i)倍
				t=(t+choose(p,i)*choose(k+n-2*p-1,n-p+i)%MOD*pom(2,p-i,MOD))%MOD;
			}
		}else{
			//偶数のとき
			//はずすべきペアが(x/2-1)組と、0or1にすべきものが1つある
			ll p=x/2-1;
			rep(i,0,p+1){
				//p組のうちちょうどi組が(0,0)だとする
				//残りの(p-i)組に先に1個わりあてると、残りの(n-p+i)個を(k-2*p+p-i)組に振り分けるので
				//まんなかを0にしたときchoose(k+n-2*p-2,n-p+i)通り
				//まんなかを1にしたときchoose(k+n-2*p-3,n-p+i-1)通り
				//(p-i)組はどっちに割り振るか各2通りあるので2^(p-i)倍
				t=(t+choose(p,i)*(choose(k+n-2*p-2,n-p+i)+choose(k+n-2*p-3,n-p+i-1))%MOD*pom(2,p-i,MOD))%MOD;
			}
		}
		ans[x]=t;
	}
	rep(i,2,k+2)printf("%lld\n",ans[i]);
	rep(i,k+2,2*k+1)printf("%lld\n",ans[2*k+2-i]);
}