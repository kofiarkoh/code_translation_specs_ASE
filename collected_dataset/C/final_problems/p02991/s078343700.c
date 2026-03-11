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
void cin(ll *n){ scanf("%lld",&(*n));}
void cin2(ll *a, ll *b){ scanf("%lld%lld",&(*a),&(*b)); }
void cin3(ll *a, ll *b, ll *c){ scanf("%lld%lld%lld",&(*a),&(*b),&(*c)); }
ll max2(ll a,ll b){return a>=b?a:b;}
ll min2(ll a,ll b){return a>=b?b:a;}
ll min3(ll a, ll b, ll c){return (a<=b && a<=c) ? a : b<=c ? b : c;}
ll max3(ll a, ll b, ll c){return (a>=b && a>=c) ? a : b>=c ? b : c;}
ll minn(ll n, ll a[n]){ll b=INF;rep(i,0,n) b=min2(b,a[i]);return b;}
ll maxn(ll n, ll a[n]){ll b=-INF;rep(i,0,n) b=max2(b,a[i]);return b;}
ll ABS(ll a){return a>=0?a:(-a);}
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

typedef struct{ int a , b;}fr;
int cmp1( const void *p, const void *q ) { return ((fr*)p) ->a - ((fr*)q)->a;}
int cmp2( const void *p, const void *q ) { return ((fr*)q) ->a - ((fr*)p)->a;}
void strsortup(fr*a,int n){qsort(a,n,sizeof(fr),cmp1);}
void strsortdown(fr*a,int n){qsort(a,n,sizeof(fr),cmp2);}


int edge_cnt[114315];
ll* graph[114315];
// ll* edge_num[114315];
ll ans[114315];
// freeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee!!!!!!!!!!!!!!!

void pushGraph(ll v1, ll v2, ll weight, ll num , ll is_dir){
    graph[v1] = realloc( graph[v1] , sizeof(ll)*(edge_cnt[v1]*2+2) );
    graph[v1][edge_cnt[v1]*2] = v2;
    graph[v1][edge_cnt[v1]*2+1] = weight;
    // edge_num[v1] = realloc( edge_num[v1] , sizeof(ll)*(edge_cnt[v1]+1) );
    // edge_num[v1][edge_cnt[v1]] = num;
    edge_cnt[v1]++;

    if(!is_dir){
        graph[v2] = realloc( graph[v2] , sizeof(ll)*(edge_cnt[v2]*2+2) );
        graph[v2][edge_cnt[v2]*2] = v1;
        graph[v2][edge_cnt[v2]*2+1] = weight;
        // edge_num[v2] = realloc( edge_num[v2] , sizeof(ll)*(edge_cnt[v2]+1) );
        // edge_num[v2][edge_cnt[v2]] = num;
        edge_cnt[v2]++;
    }
}

// int dist[1e5];

int main(void){
    // fgets(s,sizeof(s),stdin); 
    // char ;
    ll n,m,s,t;
    // ll ans=0;
    cin(&n);
    cin(&m);
    // cin2(&,&);
    // cin3(&,&,&);
    // scanf("%s",);
    ll u,v;
    rep(i,0,m){
        cin(&u);
        cin(&v);
        pushGraph(u-1,v-1,0,0,1);
    }
    // rep(i,0,n){
    //     printf("%d ",edge_cnt[i]);
    // }
    // PN;
    cin(&s);
    cin(&t);
    ll hw=1e6;
    fr que[hw];
    rep(i,0,hw) que[i].a=-1;
    que[0].a=s-1;
    que[0].b=0;
    ll head=0,tail=0;
    while( head<20*hw && que[head%hw].a!=-1 ){
        // printf("||%d %d||\n",que[head%hw].a,que[head%hw].b);
        // if( ans[que[head%hw].a] == que[head%hw].b && que[head%hw].b!=0 ){ head++; continue; }
        ans[que[head%hw].a] = que[head%hw].b;
        rep(i,0,edge_cnt[ que[head%hw].a ]){
            // if()
            if( ans[graph[que[head%hw].a][i*2]] == que[head%hw].b+1 ) continue;
            if( graph[que[head%hw].a][i*2]==t-1 && (que[head%hw].b+1)%3==0 ){
                printf("%d\n",(que[head%hw].b+1)/3 );
                return 0;
            }
            tail++;
            if(tail>20*hw-1) continue;
            que[tail%hw].a = graph[que[head%hw].a][i*2];
            que[tail%hw].b = que[head%hw].b+1;
            ans[que[tail%hw].a] = que[tail%hw].b;
            // printf("%d ",que[tail%hw].a);
        }
        // PN;
        // if(ans[t-1]!=0 && ans[t-1]%3==0){
        //     printf("%lld\n",ans[t-1]/3);
        //     return 0;
        // }
        head++;
    }

    

    
    printf("-1\n");
    return 0;
}
