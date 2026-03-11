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

typedef struct{ ll aa , bb;}fr;
int cmp1( const void *p, const void *q ) { return ((fr*)p) ->aa - ((fr*)q)->aa;}
/*nCk*/
ll nckused[1000][1000];
ll nckreached[1000][1000];
ll comb(ll n,ll k){
    if(n>1000) return (k==0 || k==n)?1:comb(n-1,k-1)+comb(n-1,k);
    if(nckreached[n][k]==1) return nckused[n][k];
    nckreached[n][k] = 1;
    nckused[n][k] = (n==1 || (k==0 || k==n) )?1:comb(n-1,k-1)+comb(n-1,k);
    return (n==1 || (k==0 || k==n) )?1:comb(n-1,k-1)+comb(n-1,k);
}

int main(void){
    // char ;
    ll n;
    scanf("%lld",&n);
    // ll [n];
    printf("%lld\n",comb(n,2)-n/2);

    if(n%2){
        rep(i,0,n-1){
            rep(j,i+1,n){
                if(j==n-2-i) continue;
                printf("%lld %lld\n",i+1,j+1);
            }
        }

    }else{
        rep(i,0,n-1){
            rep(j,i+1,n){
                if(j==n-1-i) continue;
                printf("%lld %lld\n",i+1,j+1);
            }
        }
    }

    // printf("\n");
    return 0;
}
