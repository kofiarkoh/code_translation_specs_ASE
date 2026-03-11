#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
#include <time.h>
typedef long long ll;
typedef long double ld;
#define INF (1LL<<60)
#define MOD1 1000000007
/*swap 交換*/
void swap(ll *a, ll *b){ll c;c=*b;*b=*a;*a=c;}
/*2つのうち大きい数を返す*/
ll max2(ll a,ll b){return a>=b?a:b;}
/*2つのうち小さい数を返す*/
ll min2(ll a,ll b){return a>=b?b:a;}
/*絶対値*/
ll ABS(ll a){return a>=0?a:(-a);}
typedef struct{
    ll aa;
    ll bb;
}frequent;
// 小大
int compare(const void *a, const void *b){
    return *(ll *)a>*(ll *)b?1:*(ll *)a<*(ll *)b?-1:0;}

int main(void){
    ll n,i;
    scanf("%lld",&n);
    ll a[n+2],cost=0;
    a[0]=0;
    a[n+1]=0;
    for(i=0;i<n;i++){
        scanf("%lld",&a[i+1]);
        cost+=ABS(a[i+1]-a[i]);
        // b[i]=a[i];
        // mi=min2(mi,a[i]);
        // ma=max2(ma,a[i]);
    }
    cost+=ABS(a[n+1]-a[n] );
    // qsort(b,n,sizeof(ll),compare);
    // mi2=min2(b[1],0);
    // ma2=max2(b[n-2],0);
    for(i=0;i<n;i++){
        // if(a[i]==ma){
        //     printf("%lld\n",(ma2-mi)*2 );
        // }else if(a[i]==mi){
        //     printf("%lld\n",(ma-mi2)*2 );
        // }else{
        //     printf("%lld\n",(ma-mi)*2 );
        // }
        printf("%lld\n",cost-ABS(a[i+1]-a[i])-ABS(a[i+1]-a[i+2])+ABS(a[i]-a[i+2]) );
    }
    return 0;
}
