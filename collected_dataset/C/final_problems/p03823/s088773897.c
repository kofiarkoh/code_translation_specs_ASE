//lazy segtree - set range and sum
//AGC009-C
#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<stdbool.h>
#include<time.h>
#define inf 1072114514
#define llinf 4154118101919364364
#define mod 1000000007
#define pi 3.1415926535897932384

int max(int a,int b){if(a>b){return a;}return b;}
int min(int a,int b){if(a<b){return a;}return b;}
int zt(int a,int b){return max(a,b)-min(a,b);}
int round(int a,int b){if((a%b)*2 >= b){return (a/b)+1;}return a/b;}
int ceil(int a,int b){if(a%b==0){return a/b;}return (a/b)+1;}
int gcd(int a,int b){int c;while(b!=0){c=a%b;a=b;b=c;}return a;}
int lcm(int a,int b){int c=gcd(a,b);a/=c;return a*b;}
int nCr(int a,int b){int i,r=1;for(i=1;i<=b;i++){r*=(a+1-i);r/=i;}return r;}
int nHr(int a,int b){return nCr(a+b-1,b);}
int fact(int a){int i,r=1;for(i=1;i<=a;i++){r*=i;}return r;}
int pow(int a,int b){int i,r=1;for(i=1;i<=b;i++){r*=a;}return r;}
int dsum(int x){int r=0;while(x){r+=(x%10);x/=10;}return r;}
int dsumb(int x,int b){int r=0;while(x){r+=(x%b);x/=b;}return r;}
int sankaku(int x){return ((1+x)*x)/2;}
long long llmax(long long a,long long b){if(a>b){return a;}return b;}
long long llmin(long long a,long long b){if(a<b){return a;}return b;}
long long llzt(long long a,long long b){return llmax(a,b)-llmin(a,b);}
long long llround(long long a,long long b){if((a%b)*2 >= b){return (a/b)+1;}return a/b;}
long long llceil(long long a,long long b){if(a%b==0){return a/b;}return (a/b)+1;}
long long llgcd(long long a,long long b){long long c;while(b!=0){c=a%b;a=b;b=c;}return a;}
long long lllcm(long long a,long long b){long long c=llgcd(a,b);a/=c;return a*b;}
long long llnCr(long long a,long long b){long long i,r=1;for(i=1;i<=b;i++){r*=(a+1-i);r/=i;}return r;}
long long llnHr(long long a,long long b){return llnCr(a+b-1,b);}
long long llfact(long long a){long long i,r=1;for(i=1;i<=a;i++){r*=i;}return r;}
long long llpow(long long a,long long b){long long i,r=1;for(i=1;i<=b;i++){r*=a;}return r;}
long long lldsum(long long x){long long r=0;while(x){r+=(x%10);x/=10;}return r;}
long long lldsumb(long long x,long long b){long long r=0;while(x){r+=(x%b);x/=b;}return r;}
long long llsankaku(long long x){return ((1+x)*x)/2;}
double dbmax(double a,double b){if(a>b){return a;}return b;}
double dbmin(double a,double b){if(a<b){return a;}return b;}
double dbzt(double a,double b){return dbmax(a,b)-dbmin(a,b);}
int sortfncsj(const void *a,const void *b){if(*(int *)a>*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}
int sortfnckj(const void *a,const void *b){if(*(int *)a<*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}
int llsortfncsj(const void *a,const void *b){if(*(long long *)a>*(long long *)b){return 1;}if(*(long long *)a==*(long long *)b){return 0;}return -1;}
int llsortfnckj(const void *a,const void *b){if(*(long long *)a<*(long long *)b){return 1;}if(*(long long *)a==*(long long *)b){return 0;}return -1;}
int dbsortfncsj(const void *a,const void *b){if(*(double *)a>*(double *)b){return 1;}if(*(double *)a==*(double *)b){return 0;}return -1;}
int dbsortfnckj(const void *a,const void *b){if(*(double *)a<*(double *)b){return 1;}if(*(double *)a==*(double *)b){return 0;}return -1;}
int strsortfncsj(const void *a,const void *b){return strcmp((char *)a,(char *)b);}
int strsortfnckj(const void *a,const void *b){return strcmp((char *)b,(char *)a);}

void shuffledget(int x[],int n){
    srand(time(0));
    int i,b[524288],p,c;
    for(i=0;i<n;i++){
        b[i]=i;
    }
    for(i=n;i>=1;i--){
        p=rand()%i;
        c=b[i-1];b[i-1]=b[p];b[p]=c;
    }
    for(i=0;i<n;i++){
        scanf("%d",&x[b[i]]);
    }
}

int dx4[4]={1,-1,0,0};
int dy4[4]={0,0,1,-1};
int dx8[8]={-1,-1,-1,0,0,1,1,1};
int dy8[8]={-1,0,1,-1,1,-1,0,1};

long long search(long long x,long long a[],long long n){
    int st=0,fi=n-1,te;
    while(st<=fi){
        te=(st+fi)/2;
        if(a[te]<x){st=te+1;}else{fi=te-1;}
    }
    return st;
}

typedef struct{
int val;
int node;
}sd;

int sdsortfnc(const void *a,const void *b){
if(((sd*)a)->val < ((sd*)b)->val){return -1;}
if(((sd*)a)->val > ((sd*)b)->val){return 1;}
if(((sd*)a)->node < ((sd*)b)->node){return -1;}
if(((sd*)a)->node > ((sd*)b)->node){return 1;}
return 0;
}


long long power(long long a, long long b){
	long long x = 1, y = a;
	while (b > 0){
		if(b%2){
			x = (x*y)%mod;
		}
		y = (y*y)%mod;
		b /= 2;
	}
	return x%mod;
}

long long modular_inverse(long long n){
	return power(n, mod-2);
}

long long inv2;

long long ssize=524288;
long long stree[1048576],lazy[1048576],vd=0;

void sinit(){
    inv2=modular_inverse(2);
    long long i,j;
    for(i=0;i<2*ssize;i++){
        stree[i]=0;
        lazy[i]=-llinf;
    }
    return;
}

//lazy evaluation
void eval(long long k){
  if(lazy[k]!=-llinf){
    stree[k]=lazy[k];
    stree[k]%=mod;
    if(k<(ssize-1)){
      lazy[2*k+1]=((lazy[k])*inv2);
      lazy[2*k+1]%=mod;
      lazy[2*k+2]=((lazy[k])*inv2);
      lazy[2*k+2]%=mod;
    }
    lazy[k]=-llinf;
  }
  return;
}

//set t [a,b)
//call:k=0,l=0,r=ssize
void setquery(long long a,long long b,long long t,long long k,long long l,long long r){
    long long ld,rd,md;
    long long i,lp,rp;
    eval(k);
    if(r<=a || b<=l){return;}
    if(a<=l && r<=b){
        lazy[k]=(t*(r-l));
        lazy[k]%=mod;
        eval(k);
        return;
    }
    else{
        eval(k*2+1);
        eval(k*2+2);
        setquery(a,b,t,k*2+1,l,(l+r)/2);
        setquery(a,b,t,k*2+2,(l+r)/2,r);
        stree[k]=(stree[k*2+1]+stree[k*2+2])%mod;
        lazy[k]=-llinf;
        return;
    }
}

//calc sum[a,b)
//call:k=0,l=0,r=ssize
long long sumquery(long long a,long long b,long long k,long long l,long long r){
    long long ld,rd,md;
    long long i,lp,rp;
    eval(k);
    if(r<=a || b<=l){return vd;}
    if(a<=l && r<=b){
        return stree[k];
    }
    else{
        ld=sumquery(a,b,k*2+1,l,(l+r)/2);
        rd=sumquery(a,b,k*2+2,(l+r)/2,r);
        lp=0;rp=0;
        md=ld+rd;
        md%=mod;
        return md;
    }
}

int main(void){
    sinit();
    long long i,j,n,m,k,a,b,c,h,w,r=0,l,t;
    long long arr[524288];
    long long ptr=262144;
    long long ak,bk,st,fi,te;
    scanf("%lld%lld%lld",&n,&a,&b);
    arr[0]=-llinf;
    for(i=1;i<=n;i++){
      scanf("%lld",&arr[i]);
    }
    setquery(0,0+1,1,0,0,ssize);
    setquery(ptr+0,(ptr+0)+1,1,0,0,ssize);
    for(i=2;i<=n;i++){
      st=0;fi=i;
      while(st<=fi){
        te=(st+fi)/2;
        if(arr[i]-arr[te]>=a){
          st=te+1;
        }
        else{
          fi=te-1;
        }
      }
      ak=fi;

      st=0;fi=i;
      while(st<=fi){
        te=(st+fi)/2;
        if(arr[i]-arr[te]>=b){
          st=te+1;
        }
        else{
          fi=te-1;
        }
      }
      bk=fi;

      w=sumquery(ptr,(ptr+llmin(i-2,ak))+1,0,0,ssize);
      setquery(i-1,(i-1)+1,w,0,0,ssize);
      //printf("%lld:%lld,<<%lld>>\n",i,ak,w);

      w=sumquery(0,llmin(i-2,bk)+1,0,0,ssize);
      setquery(ptr+i-1,(ptr+i-1)+1,w,0,0,ssize);
      //printf("%lld:%lld,[[%lld]]\n",i,bk,w);

      if(ak!=i-1){
        setquery(0,(i-2)+1,0,0,0,ssize);
      }
      if(bk!=i-1){
        setquery(ptr,(ptr+i-2)+1,0,0,0,ssize);
      }

      //for(j=0;j<=i;j++){
      //  printf("<%lld>",sumquery(j,j+1,0,0,ssize));
      //}printf("\n");

      //for(j=0;j<=i;j++){
      //  printf("[%lld]",sumquery(ptr+j,ptr+j+1,0,0,ssize));
      //}printf("\n");
      
    }
    printf("%lld\n",sumquery(0,ssize,0,0,ssize));
    return 0;
}
