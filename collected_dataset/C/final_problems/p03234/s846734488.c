//set many funcs template
//Ver.20190820
#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<stdbool.h>
#include<time.h>
#include<assert.h>
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
void swap(int *a,int *b){int c;c=(*a);(*a)=(*b);(*b)=c;}
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
void llswap(long long *a,long long *b){long long c;c=(*a);(*a)=(*b);(*b)=c;}
double dbmax(double a,double b){if(a>b){return a;}return b;}
double dbmin(double a,double b){if(a<b){return a;}return b;}
double dbzt(double a,double b){return dbmax(a,b)-dbmin(a,b);}
void dbswap(double *a,double *b){double c;c=(*a);(*a)=(*b);(*b)=c;}
void chswap(char *a,char *b){char c;c=(*a);(*a)=(*b);(*b)=c;}
int sortfncsj(const void *a,const void *b){if(*(int *)a>*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}
int sortfnckj(const void *a,const void *b){if(*(int *)a<*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}
int llsortfncsj(const void *a,const void *b){if(*(long long *)a>*(long long *)b){return 1;}if(*(long long *)a==*(long long *)b){return 0;}return -1;}
int llsortfnckj(const void *a,const void *b){if(*(long long *)a<*(long long *)b){return 1;}if(*(long long *)a==*(long long *)b){return 0;}return -1;}
int dbsortfncsj(const void *a,const void *b){if(*(double *)a>*(double *)b){return 1;}if(*(double *)a==*(double *)b){return 0;}return -1;}
int dbsortfnckj(const void *a,const void *b){if(*(double *)a<*(double *)b){return 1;}if(*(double *)a==*(double *)b){return 0;}return -1;}
int strsortfncsj(const void *a,const void *b){return strcmp((char *)a,(char *)b);}
int strsortfnckj(const void *a,const void *b){return strcmp((char *)b,(char *)a);}
int chsortfncsj(const void *a,const void *b){if(*(char *)a>*(char *)b){return 1;}if(*(char *)a==*(char *)b){return 0;}return -1;}
int chsortfnckj(const void *a,const void *b){if(*(char *)a<*(char *)b){return 1;}if(*(char *)a==*(char *)b){return 0;}return -1;}

void shuffledget(int x[],int n){
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

int search(int x,int a[],int n){
    int st=0,fi=n-1,te;
    while(st<=fi){
        te=(st+fi)/2;
        if(a[te]<x){st=te+1;}else{fi=te-1;}
    }
    return st;
}

void prarr(int arr[],int n){
  int i;
  for(i=0;i<n;i++){
    if(i){printf(" ");}
    printf("%d",arr[i]);
  }
  printf("\n");
  return;
}

void getperm(int a[],int n){
  int i,p;
  for(i=0;i<n;i++){
    a[i]=i;
  }
  for(i=n-1;i>=1;i--){
    p=rand()%(i+1);
    swap(&a[p],&a[i]);
  }
}

typedef struct{
int val;
int node;
}sd;

int sdsortfnc(const void *a,const void *b){
if(((sd*)a)->val < ((sd*)b)->val){return -1;}
if(((sd*)a)->val > ((sd*)b)->val){return 1;}
return 0;
}

void coordinate_comp(int a[],int n){
  int i,c=0;
  sd dat[524288];
  for(i=0;i<n;i++){
    dat[i].val=a[i];
    dat[i].node=i;
  }
  qsort(dat,n,sizeof(dat[0]),sdsortfnc);
  a[dat[0].node]=c;
  for(i=1;i<n;i++){
    if(dat[i-1].val!=dat[i].val){c++;}
    a[dat[i].node]=c;
  }
}

long long ssize=1024;
long long stree[2][2048];
long long vd[2];

void sinit(){
    long long i;
    vd[0]=llinf;
    vd[1]=-llinf;
    for(i=0;i<2*ssize;i++){
        stree[0][i]=llinf;
        stree[1][i]=-llinf;
    }
}

void upd(long long k,long long a){
    k+=ssize-1;
    stree[0][k]=a;
    stree[1][k]=a;
    while(k>0){
        k=(k-1)/2;
        stree[0][k]=llmin(stree[0][2*k+1],stree[0][2*k+2]);
        stree[1][k]=llmax(stree[1][2*k+1],stree[1][2*k+2]);
    }
}

//calc min[a,b)(t=0)
//calc max[a,b)(t=1)
//call:k=0,l=0,r=ssize
long long query(long long a,long long b,long long t,long long k,long long l,long long r){
    long long ld,rd;
    if(r<=a || b<=l){
      return vd[t];
    }
    if(a<=l && r<=b){
        return stree[t][k];
    }
    else{
        ld=query(a,b,t,k*2+1,l,(l+r)/2);
        rd=query(a,b,t,k*2+2,(l+r)/2,r);
        if(t==0){return llmin(ld,rd);}
        else{return llmax(ld,rd);}
    }
}

//Binary Indexed Tree
long long bit[1116]={0},bsize=1024;
long long sum(long long i){
    long long s=0;
    while(i>0){
        s+=bit[i];
        i-=i&(-i);
    }
    return s;
}
void add(long long i,long long x){
    while(i<=bsize){
        bit[i]+=x;
        i+=i&(-i);
    }
}

long long dp[666][666];
long long pat[666]={0};
long long pr[666]={0};

long long rep(long long l,long long r){
  if(r-l<=0){return 0;}
  if(dp[l][r]!=-1){return dp[l][r];}
  long long i,c;
  if(query(l,r+1,0,0,0,ssize)<l){return 0;}
  if(query(l,r+1,1,0,0,ssize)>r){return 0;}
  if(l+1==r){return 1;}
  dp[l][r]=0;
  c=sum(r)-sum(l-1);
  if(c%2==0){
    dp[l][r]=pat[c];
    for(i=l;i<r;i++){
      if(pr[i]==0){c--;}
      if/*(c%2==1)*/(c&1ll){continue;}
      dp[l][r]=(dp[l][r]+mod-((rep(l,i)*pat[c])%mod))%mod;
    }
  }
  return dp[l][r];
}

int main(void){
  sinit();
  long long i,j,n,k,a,b,r=0;
  scanf("%lld%lld",&n,&k);
  pat[0]=1;
  pat[1]=0;
  for(i=2;i<666;i++){
    pat[i]=(pat[i-2]*(i-1))%mod;
  }
  for(i=1;i<=2*n;i++){add(i,1);}
  for(i=0;i<k;i++){
    scanf("%lld%lld",&a,&b);
    pr[a]=b;
    pr[b]=a;
    upd(a,b);
    upd(b,a);
    add(a,-1);
    add(b,-1);
  }
  for(i=0;i<666;i++){
    for(j=0;j<666;j++){
      dp[i][j]=-1;
    }
  }
  for(i=1;i<=2*n;i++){
    for(j=i+1;j<=2*n;j++){
      //printf("%lld %lld : %lld\n",i,j,rep(i,j));
      r=(r+rep(i,j)*pat[2*(n-k)-(sum(j)-sum(i-1))])%mod;
    }
  }
  printf("%lld\n",r);
  return 0;
}
