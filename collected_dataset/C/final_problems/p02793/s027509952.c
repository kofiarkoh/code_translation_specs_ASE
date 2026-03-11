#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
#include<stdbool.h>
#include<assert.h>
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
#define charsize 100005 //10^5+5
#define PI 3.141592653589793238

void swap(ll *a, ll *b){ll c;c=*b;*b=*a;*a= c;}
void cin(ll *n){ scanf("%lld",&(*n)); }
ll max2(ll a,ll b){return a>=b?a:b;}
ll min2(ll a,ll b){return a>=b?b:a;}
ll min3(ll a, ll b, ll c){return (a<=b && a<=c) ? a : b<=c ? b : c;}
ll max3(ll a, ll b, ll c){return (a>=b && a>=c) ? a : b>=c ? b : c;}
ll minn(ll n, ll a[]){ll b=INF;rep(i,0,n) b=min2(b,a[i]);return b;}
ll maxn(ll n, ll a[]){ll b=-INF;rep(i,0,n) b=max2(b,a[i]);return b;}
ll POW(ll a, ll b){ll c=1;rep(i,0,b) c*=a;return c;}
double POW_d(double a, double b){double c=1;rep(i,0,b) c*=a;return c;}
ll gcd(ll a,ll b){return b?gcd(b,a%b):a;}
ll lcm(ll a,ll b){return a/gcd(a,b)*b;}
ll mod_MOD1(ll n){n+= n<0?((-n)/MOD1+1)*MOD1:0; return n%=MOD1;}
ll mod_p(ll n ,ll p){n+= n<0?((-n)/p+1)*p:0; return n%=p;}
ll change_into_num(char s[] , ll len, ll p){ return !p ? 0 : POW(10,p-1)*(s[len-p]-'0') + change_into_num(s,len,p-1); }

void lr_lower( int *l, int *r, ll am, ll val , int type ){ (type<3) ? ( am < val ?  ( *l = (*l+*r)/2 ) : ( *r= (*l+*r)/2 ) ) : ( am <= val ? ( *l = (*l+*r)/2 ) : ( *r= (*l+*r)/2 ) ); }
void lr_upper( int *l, int *r, ll am, ll val , int type ){ (type<3) ? ( am <= val ?  ( *l = (*l+*r)/2 ) : ( *r= (*l+*r)/2 ) ) : ( am < val ? ( *l = (*l+*r)/2 ) : ( *r= (*l+*r)/2 ) ); }
int cmp_lower( ll a, ll b, int type ){ return (type==1) ? ( a==b ? 1 : 0 ) : (type==2) ? ( a>=b ? 1 : 0 ) : ( a>b ? 1 : 0 ) ; }
int cmp_upper( ll a, ll b, int type ){ return (type==1) ? ( a==b ? 1 : 0 ) : (type==2) ? ( a<=b ? 1 : 0 ) : ( a<b ? 1 : 0 ) ; }
// return smallest p  which meets  a[p]==val :1  >=:2   >:3
ll lower_bound( ll a[], int l, int r, ll val , int type ){  while(r-l>1) lr_lower(&l,&r,a[ (l+r)/2 ],val,type);  return cmp_lower(a[l],val,type) ? l : cmp_lower(a[r],val,type) ? r : -1;  }
// return biggest p  which meets   a[p]==val :1  <=:2   <:3
ll upper_bound( ll a[], int l, int r, ll val , int type ){  while(r-l>1) lr_upper(&l,&r,a[ (l+r)/2 ],val,type); return cmp_upper(a[r],val,type) ? r : cmp_upper(a[l],val,type) ? l : -1; }
// count i  which meets ai==x
ll count(ll a[], int l, int r, ll x){  int p = lower_bound(a,l,r,x,1);  return p==-1 ? 0 : upper_bound(a,p,r,x,1)-p+1; }
ll *factors[2] , fac_cnt=0;
ll factor_pre(ll x){  rep(i,0,2){ factors[i] = realloc( factors[i] , sizeof(ll) ); factors[i][i?fac_cnt++:fac_cnt]=i?0:2; } repp(i,3,x+1,2) rep(j,0,fac_cnt) if( !(i%factors[0][j]) ){break;} else if(POW(factors[0][j],2)>i){  rep(k,0,2){ factors[k] = realloc( factors[k] , sizeof(ll)*( fac_cnt +1 ) );  factors[k][k?fac_cnt++:fac_cnt]=k?0:i;  } break; }  return 0; }
ll factor(ll n, ll is_new){ 
    rep(i,0,fac_cnt){ 
        ll cnt=0; 
        rep(j,0,1){ 
            while( ( cnt+= n %factors[0][i]==0 ? 1 : 0 )  && (n/=factors[0][i]) %factors[0][i]==0 ) continue;
        } 
        factors[1][i]= is_new ? cnt : max2(factors[1][i],cnt);  
        if( factors[0][i]/**factors[0][i]*/ > n ) break; 
    }    
return n; 
}
ll judge_prime(ll n){ rep(i,0,fac_cnt){ if(n<factors[0][i]*factors[0][i] || n==factors[0][i]) break; else if(n%factors[0][i]==0) n/=n; }  return n==1?0:1; }


int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
int cmp_string( const void * a , const void * b ) {  return strcmp( (char *)a , (char *)b ); }  // qsort((void*)s,n,sizeof(s[0]),int_sort );
int cmp_char(const void * a, const void * b) { return *(char *)a - *(char *)b;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}
void sort_string(int n,int size,char s[][size]){ qsort( (void*)s , n , sizeof(s[0]) , cmp_string ); }
void sort_char(char *s){ qsort( (void *)s , strlen(s) , sizeof(char) , cmp_char ); }
ll unique_string(ll n ,ll size, char s[][size]){ ll ans=1; rep(i,1,n) if( strcmp(s[i],s[i-1]) ) ans++; return ans; }
ll unique_num(ll n , ll a[]){ ll ans=1; rep(i,1,n) if( a[i]!=a[i-1] ) ans++; return ans; }

typedef struct{ ll a , b;}fr;
int cmp1( const void *p, const void *q ) { return ((fr*)p) ->a - ((fr*)q)->a;}
int cmp2( const void *p, const void *q ) { return ((fr*)q) ->a - ((fr*)p)->a;}
void strsortup(fr*a,int n){qsort(a,n,sizeof(fr),cmp1);}
void strsortdown(fr*a,int n){qsort(a,n,sizeof(fr),cmp2);}


ll inv[2000005],finv[2000005],mod_fac[2000005];
void makeinv(ll n){  inv[1]=1;finv[0]=finv[1]=1;  rep(i,2,n+1){  inv[i]= MOD1 - inv[MOD1%i] * (MOD1 / i) % MOD1;   finv[i] = finv[i - 1] * inv[i] % MOD1;  }  }
void make_mod_fac(ll n, ll mod){ ll x=1; mod_fac[0]=mod_fac[1]=x; rep(i,2,n+1){ x*=i%mod; x%=mod; mod_fac[i]=x; } }
ll mod_nck(ll n, ll k, ll mod){ return mod_fac[n]*finv[k]%mod*finv[n-k]%mod; }
ll modpow(ll r,ll n){  ll t=1,s=r;  while(n>0){ t = (n&1) ? t*s%MOD1 : t;  s=s*s%MOD1; n>>=1; }  return r?t:0;  }

ll ano[10000];

int main(void){
    // fgets(s,sizeof(s),stdin); 
    // char ;
    ll n;
    ll ans=0;
    cin(&n);
    // scanf("%s",);
    factor_pre(1e3);
    makeinv(1e6);
    ll a[n];
    ll tmp=1,mark=0;
    rep(i,0,n){
        cin(&a[i]);
        ll piv=factor(a[i],0);
        if(piv!=1){
            ano[mark++]=piv;
        }
    }
    // rep(i,0,10) printf("%lld %lld\n",factors[0][i],factors[1][i]);
    rep(i,0,fac_cnt){
        // if(factors[1][i]) printf("%lld %lld\n",factors[0][i],factors[1][i]);
        tmp*=modpow(factors[0][i],factors[1][i]);
        tmp%=MOD1;
    }
    if(mark){
        sortdown(ano,10000);
        tmp*=ano[0]; tmp%=MOD1;
        rep(i,1,mark){
            if(ano[i]!=ano[i-1]){
                tmp*=ano[i];
                tmp%=MOD1;
            }
        }
    }
    // printf("%lld\n",tmp);
    rep(i,0,n){
        ans+= mod_MOD1(tmp*inv[a[i]]);
        // ans+= tmp/a[i];
        ans%=MOD1;
    }

    printf("%lld\n",ans);
    return 0;
}
