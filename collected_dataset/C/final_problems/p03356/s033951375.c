#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
#include<stdbool.h>
typedef long long ll;
typedef long double ld;
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define rrep(i,l,r)for(ll i=(l);i>=(r);i--)
#define INF (1LL<<60)
#define MOD1 1000000007
#define MOD2 998244353
#define MAX_N (1 << 17)
#define YES printf("Yes\n")
#define NO printf("No\n")
#define PN printf("\n")

void swap(ll *a, ll *b){ll c;c=*b;*b=*a;*a=c;}
ll max2(ll a,ll b){return a>=b?a:b;}
ll min2(ll a,ll b){return a>=b?b:a;}
ll min3(ll a, ll b, ll c){return (a<=b && a<=c) ? a : b<=c ? b : c;}
ll max3(ll a, ll b, ll c){return (a>=b && a>=c) ? a : b>=c ? b : c;}
ll minn(ll n, ll a[n]){ll b=INF;rep(i,0,n) b=min2(b,a[i]);return b;}
ll maxn(ll n, ll a[n]){ll b=-INF;rep(i,0,n) b=max2(b,a[i]);return b;}
ll ABS(ll a){return a>=0?a:(-a);}
ll POW(ll a, ll b){ll c=1;rep(i,0,b) c*=a;return c;}
ll gcd(ll a,ll b){return b?gcd(b,a%b):a;}
ll lcm(ll a,ll b){return a/gcd(a,b)*b;}
ll mod(ll n){n+= n<0?((-n)/MOD1+1)*MOD1:0; return n%=MOD1;}
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}

typedef struct{ ll a , b;}fr;
int cmp1( const void *p, const void *q ) { return ((fr*)p) ->a - ((fr*)q)->a;}
int cmp2( const void *p, const void *q ) { return ((fr*)q) ->a - ((fr*)p)->a;}
void strsortup(fr*a,int n){qsort(a,n,sizeof(fr),cmp1);}
void strsortdown(fr*a,int n){qsort(a,n,sizeof(fr),cmp2);}

/*-------------------------------------------------*/
// Union Find              添字に注意!!!!!!!!!!
typedef struct{
    ll num; //number
    ll par; //parent
    ll height; //height
}UF;
void UFmake(ll n, UF b[n]){for(ll i=0;i<n;i++){
    b[i].par = i;
    b[i].num = i;
    b[i].height = 1;
    }
}
ll UFroot(ll n, ll a,UF b[n]){ return a==b[a].par?a:UFroot(n,b[a].par,b); }
void UFunite(ll n, ll x, ll y, UF b[n]){
    if(b[UFroot(n,x,b)].height > b[UFroot(n,y,b)].height) b[UFroot(n,y,b)].par = UFroot(n,x,b);
    else{ b[UFroot(n,x,b)].par = UFroot(n,y,b); b[UFroot(n,y,b)].height = max2(b[UFroot(n,y,b)].height,b[UFroot(n,x,b)].height+1);}
}
ll UFsame(ll n, ll x, ll y, UF b[n]){ return UFroot(n,x,b) == UFroot(n,y,b) ? 1 : 0; }

/*-------------------------------------------------------------*/

int main(void){
    // char ;
    ll n,m,ans=0;
    scanf("%lld%lld",&n,&m);
    ll p[n],x,y;
    rep(i,0,n){
        scanf("%lld",&p[i]);
    }
    UF seq[n];
    UFmake(n,seq);
    rep(i,0,m){
        scanf("%lld%lld",&x,&y);
        UFunite(n,x-1,y-1,seq);
    }
    rep(i,0,n){
        if( UFsame(n,i,p[i]-1,seq) ){
            ans++;
        }
    }
    printf("%lld\n",ans);
    return 0;
}
