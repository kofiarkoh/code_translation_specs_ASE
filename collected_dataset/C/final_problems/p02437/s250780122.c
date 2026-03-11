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

typedef struct{ ll a , b;}fr;
int cmp1( const void *p, const void *q ) { return ((fr*)p) ->a - ((fr*)q)->a;}
int cmp2( const void *p, const void *q ) { return ((fr*)q) ->a - ((fr*)p)->a;}
void strsortup(fr*a,int n){qsort(a,n,sizeof(fr),cmp1);}
void strsortdown(fr*a,int n){qsort(a,n,sizeof(fr),cmp2);}

/*----------------------priorityqueue-----------------------------------------------------------*/
// kind = max?1:0
ll HeapValue(ll a){
    return a?-INF:INF;
}
ll Heapcmp(ll a, ll b, ll kind){
    return kind ? (a<b?1:0) : (a>b?1:0);
}
ll heap[200000][1000];
void pushHeap(ll set, ll val, ll len , ll kind){
    ll i=len; heap[set][i]=val;
    while(i!=0){
        if( Heapcmp( heap[set][i], heap[set][(i-1)/2] , ABS(kind-1) )  ) swap(&heap[set][i],&heap[set][(i-1)/2]);
        i--;i/=2;
    }
}
void popHeap(ll set, ll *val, ll len, ll kind){
    ll i=0; *val=heap[set][0]; heap[set][0]=HeapValue(kind);
    swap(&heap[set][0],&heap[set][len-1]);
    while(len>i*2+1){
        if(i*2+2<len && Heapcmp(heap[set][i],heap[set][i*2+2],kind) && Heapcmp(heap[set][i*2+1],heap[set][i*2+2],kind) ){
            swap(&heap[set][i],&heap[set][i*2+2]);
            i*=2;i+=2;
        }else{
            if( Heapcmp(heap[set][i],heap[set][i*2+1],kind) ){
                swap(&heap[set][i],&heap[set][i*2+1]);
                i*=2;i++;
            }else break;
        }
    }
}
/*---------------------------------------------------------------------------------*/

int main(void){
    // char ;
    ll n,q;
    scanf("%lld%lld",&n,&q);
    ll len[n],set[q],ope[q],x[q];
    rep(i,0,n) len[i]=0;
    rep(i,0,q){
        scanf("%lld%lld",&ope[i],&set[i]);
        if(ope[i]==0){
            scanf("%lld",&x[i]);
        }
    }
    rep(i,0,q){
        if(ope[i]==0){
            pushHeap(set[i],x[i],len[set[i]],1);
            len[set[i]]++;
        }
        if(ope[i]==1){
            if(len[set[i]]==0) continue;
            printf("%lld\n",heap[set[i]][0]);
        }
        if(ope[i]==2){
            if(len[set[i]]==0) continue;
            ll a;
            popHeap(set[i],&a,len[set[i]],1);
            len[set[i]]--;
        }
        // printf("%lld  ",len[0]);
        // rep(j,0,len[0]){
        //     printf("%lld ",heap[0][j]);
        // }
        // PN;
    }
    // printf("\n");
    return 0;
}

