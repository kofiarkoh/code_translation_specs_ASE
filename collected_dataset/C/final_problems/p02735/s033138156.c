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
#define MAX_N (1 << 17)
#define YES printf("Yes\n")
#define NO printf("No\n")


void swap(ll *a, ll *b){ll c;c=*b;*b=*a;*a=c;}
ll max2(ll a,ll b){return a>=b?a:b;}
ll min2(ll a,ll b){return a>=b?b:a;}
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


int main(void){
    char s[105][105];
    ll h,w;
    scanf("%lld%lld",&h,&w);
    // ll [n];
    rep(i,0,h){
        scanf("%s",s[i]);
    }
    ll dp[100][100],dx[2]={1,0},dy[2]={0,1};
    rep(i,0,h) rep(j,0,w) dp[i][j]=INF;
    
    rep(i,0,h){
        rep(j,0,w){
            if(i==0&&j==0){
                dp[i][j]= s[0][0]=='.'?0:1;
            }
            rep(k,0,2){
                ll X=i+dx[k],Y=j+dy[k];
                if( (X>=0&&X<w) && (Y>=0&&Y<h) ){
                    if(s[X][Y]=='.'){
                        dp[X][Y]=min2(dp[X][Y],dp[i][j]);
                    }else{
                        if(s[i][j]=='#'){
                            dp[X][Y]=min2(dp[X][Y],dp[i][j]);
                        }else{
                            dp[X][Y]=min2(dp[X][Y],dp[i][j]+1);
                        }
                    }
                }
            }
        }
    }
    printf("%lld\n",dp[h-1][w-1]);
    return 0;
}
