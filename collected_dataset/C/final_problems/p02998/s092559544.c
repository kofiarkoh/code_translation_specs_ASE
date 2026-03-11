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
ll mod(ll n){return n%=MOD1;}
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}


/*ai<=xとなる最大のi*/
ll lower_bound4(ll n, ll a[n][2], ll l, ll r, ll x){
    while(r-l>1){
        ll p=(l+r)/2;
        if(a[p][0]<=x) l=p;
        else r=p;
    }
    return a[r][0]<=x?r:a[l][0]<=x?l:n;
}

/*ai<xとなる最大のi*/
ll lower_bounddash(ll n, ll a[n][2], ll l, ll r, ll x){
    while(r-l>1){
        ll p=(l+r)/2;
        if(a[p][0]<x) l=p;
        else r=p;
    }
    return a[r][0]<x?r:a[l][0]<x?l:n;
}

/*ai<xとなる最大のi*/
ll lower_bound(ll n, ll a[n], ll l, ll r, ll x){
    while(r-l>1){
        ll p=(l+r)/2;
        if(a[p]<x) l=p;
        else r=p;
    }
    return a[r]<x?r:a[l]<x?l:-1;
}

/*ai>xとなる最小のi*/
ll upper_bound(ll n, ll a[n], ll l, ll r, ll x){
    while(r-l>1){
        ll p=(l+r)/2;
        if(a[p]<=x) l=p;
        else r=p;
    }
    return a[l]>x?l:a[r]>x?r:n;
}

ll trans=1000000;
ll xkind,ykind,pkind;
ll xused[100000]={},yused[100000]={},reached[100000][2]={},piv[100005]={};

void dfs(ll n, ll xbase[n], ll ybase[n], ll x, ll y){
    if(xused[x-1]==0) xkind++;
    if(yused[y-1]==0) ykind++;
    ll zzz=lower_bound4(n,reached,piv[x-1],n-1,x*trans+y);
    reached[zzz][1]=1;
    pkind++;
    // xkind++; ykind++;
    xused[x-1]=1; yused[y-1]=1;

    ll l=upper_bound(n,xbase,0,n-1,x*trans-1) ,r=lower_bound(n,xbase,0,n-1,(x+1)*trans) ,
    d=upper_bound(n,ybase,0,n-1,y*trans-1) ,u=lower_bound(n,ybase,0,n-1,(y+1)*trans);

    // printf("x;%lld y;%lld %lld %lld\n",x,y,l,r);
    ll piv1=lower_bound4(n,reached,piv[x-1],n-1,x*trans+ xbase[l]%trans );
    rep(i,l,r+1){
        ll Y=xbase[i]%trans;
        if(Y==y) continue;
        if( reached[ piv1+ (i-l) ][1] == 0 ){
            dfs(n,xbase,ybase,x,Y);
        }
    }
    // ll piv2=lower_bound4(n,reached,0,n-1,X*trans+y)
    rep(i,d,u+1){
        ll X=ybase[i]%trans;
        if(X==x) continue;
        if( reached[ lower_bound4(n,reached,piv[X-1],n-1,X*trans+y) ][1] == 0 ){
            dfs(n,xbase,ybase,X,y);
        }
    }
}

int main(void){
    ll n,ans=0;
    scanf("%lld",&n);
    ll xbase[n],ybase[n];
    rep(i,0,n){
        ll x,y;
        scanf("%lld%lld",&x,&y);
        xbase[i]=x*trans+y;
        ybase[i]=y*trans+x;
    }
    sortup(xbase,n);
    sortup(ybase,n);
    rep(i,0,n) reached[i][0]=xbase[i];
    rep(i,1,100005){
        piv[i]=lower_bounddash(n,reached,piv[i-1],n-1,(i+1)*trans);
    }
    // rep(i,0,10) printf("%lld ",reached[i][1]);
    // PN;
    // rep(i,0,10) printf("%lld ",piv[i]);

    rep(i,0,n){
        // printf("%lld\n",ybase[i]);
        if( xused[ybase[i]%trans-1] ) continue;
        xkind=0; ykind=0; pkind=0;
        dfs(n,xbase,ybase, ybase[i]%trans , ybase[i]/trans );
        if(xkind<2 || ykind<2) continue;
        else ans+= xkind * ykind - pkind;
        // printf("%lld %lld %lld\n",xkind,ykind,pkind);
        // printf("%lld\n",ans);
    }
    
    printf("%lld\n",ans);
    return 0;
}
