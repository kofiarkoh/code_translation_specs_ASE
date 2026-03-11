#include<stdio.h>
#include<string.h>
#pragma GCC optimize("O3")
#pragma GCC target("sse,sse2,sse3,ssse3,sse4,popcnt,abm,mmx,avx,tune=native")
static inline void PUT(char c)
{
  static char buf[1<<15],*ptr=buf;
  if(ptr==buf+strlen(buf)||c==0){fwrite(buf,1,ptr-buf,stdout),ptr=buf;}*ptr++=c;
}
static inline int IN(void)
{
  int x=0,f=0,c=getchar();while(c<48||c>57){f^=c==45,c=getchar();}
  while(c>47&&c<58){x=x*10+c-48,c=getchar();}return f?-x:x;
}
static inline void OUT(int a)
{
  if(a<0)PUT('-'),a=-a;
  int d[40],i=0;do{d[i++]=a%10;}while(a/=10);
  while(i--){PUT(d[i]+48);}PUT('\n');
}
static inline int Max(int a,int b){return a>b?a:b;}
static inline int Min(int a,int b){return a<b?a:b;}
#define rep(i,n) FOR(i,0,n-1)
#define FOR(i,a,b) for(int i=a;i<=b;i++)
const int INF=0x7f7f7f7f;
int k,n,sqrtn=0,dat[500000],buk[707]={0},lazy[707]={0},lazyflag[707]={0};
static inline void init(int x)
{
  n=x;
  while(sqrtn*sqrtn<=n){sqrtn++;}
  k=(n+sqrtn-1)/sqrtn;
  rep(i,n){dat[i]=i;}
}
static inline void eval(int b)
{
  if(!lazyflag[b]){return;}
  lazyflag[b]=0;
  FOR(i,sqrtn*b,sqrtn*(b+1)-1){dat[i]=lazy[b];}
}
static inline void update(int l,int r,int x)
{
  rep(b,k)
  {
    int p=sqrtn*b,q=sqrtn*(b+1);
    if(q<=l||r<=p){continue;}
    if(l<=p&&q<=r){buk[b]=p-x;lazyflag[b]=1;lazy[b]=x;}
    else
    {
      eval(b);
      FOR(i,Max(l,p),Min(r,q)-1){dat[i]=x;}
      buk[b]=INF;
      FOR(i,p,q-1){buk[b]=Min(buk[b],i-dat[i]);}
    }
  }
}
static inline int query(int l,int r)
{
  int ret=INF;
  rep(b,k)
  {
    int p=sqrtn*b,q=sqrtn*(b+1);
    if(q<=l||r<=p){continue;}
    if(l<=p&&q<=r){ret=Min(ret,buk[b]);}
    else{eval(b);FOR(i,Max(l,p),Min(r,q)-1){ret=Min(ret,i-dat[i]);}}
  }
  return ret;
}
int main(void)
{
  int H=IN(),W=IN();
  init(W);
  rep(i,H)
  {
    int l=IN()-1,r=IN(),u=-INF;
    if(l>0){int b=(l-1)/sqrtn;eval(b);u=dat[l-1];}
    update(l,r,u);
    int ans=query(0,W);
    if(ans>=INF){ans=-1;}else{ans+=i+1;}
    OUT(ans);
  }
}