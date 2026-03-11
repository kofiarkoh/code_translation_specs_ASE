#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
typedef long long ll;
typedef long double ld;
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF (1LL<<60)
#define MOD1 1000000007

void swap(ll *a, ll *b){ll c;c=*b;*b=*a;*a=c;}/*swap 交換*/
ll max2(ll a,ll b){return a>=b?a:b;}/*2つのうち大きい数を返す*/ 
ll min2(ll a,ll b){return a>=b?b:a;}/*2つのうち小さい数を返す*/ 
ll ABS(ll a){return a>=0?a:(-a);}/*絶対値*/ 
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}

typedef struct{
    ll num; //number
    ll par; //parent
    ll height; //height
    ll wdif;
    ll weight;
}WUF;

void WUFmake(ll n, WUF b[n]){for(ll i=0;i<n;i++){
    b[i].par = i; b[i].num = i; b[i].height = 1; b[i].wdif = 0; b[i].weight = 0;}}

ll WUFroot(ll n, ll a, WUF b[n]){
    // b[a].weight = b[b[a].par].weight + b[a].wdif;
    // return a==b[a].par ? a : WUFroot(n,b[a].par,b) ;
    ll c= a==b[a].par ? a : WUFroot(n,b[a].par,b) ;
    b[a].weight = b[b[a].par].weight + b[a].wdif;
    return c;
}

// b[x]+z=b[y]
void WUFunite(ll n, ll x, ll y, ll z, WUF b[n]){
    ll xroot = WUFroot(n,x,b), yroot = WUFroot(n,y,b);
    if(b[xroot].height > b[yroot].height){
        b[yroot].par = xroot;
        b[yroot].wdif = b[x].weight+z-b[y].weight;
    }else{
        b[xroot].par = yroot;
        b[yroot].height = max2(b[yroot].height,b[xroot].height+1);
        b[xroot].wdif = b[y].weight-b[x].weight-z;
    }
}

ll WUFsame(ll n, ll x, ll y, WUF b[n]){
    return WUFroot(n,x,b) == WUFroot(n,y,b) ? 1 : 0; 
    // if( WUFroot(n,x,b) == WUFroot(n,y,b) ){
    //     return 1;
    // }
    // return 0;
}


int main(void){
    ll n,q,i;
    scanf("%lld%lld",&n,&q);
    ll query[q],x[q],y[q],z[q];
    WUF tree[n];
    WUFmake(n,tree);
    rep(i,0,q){
        scanf("%lld",&query[i]);
        if(query[i]==0){
            scanf("%lld%lld%lld",&x[i],&y[i],&z[i]);
        }else{
            scanf("%lld%lld",&x[i],&y[i]);
        }
    }
    rep(i,0,q){
        if(query[i]==0){
            if(!WUFsame(n,x[i],y[i],tree)){
                WUFunite(n,x[i],y[i],z[i],tree);
            }
        }else{
            if(WUFsame(n,x[i],y[i],tree)){
                printf("%lld\n",tree[y[i]].weight-tree[x[i]].weight);
            }else{
                printf("?\n");
            }
        }
    }
    return 0;
}
