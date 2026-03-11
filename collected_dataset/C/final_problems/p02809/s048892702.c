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

int fact(int a){int i,r=1;for(i=1;i<=a;i++){r*=i;}return r;}
void swap(int *a,int *b){int c;c=(*a);(*a)=(*b);(*b)=c;}
long long llmax(long long a,long long b){if(a>b){return a;}return b;}
long long llmin(long long a,long long b){if(a<b){return a;}return b;}
long long llzt(long long a,long long b){return llmax(a,b)-llmin(a,b);}
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

void prarr(int arr[],int n){
  int i;
  for(i=0;i<n;i++){
    if(i){printf(" ");}
    printf("%d",arr[i]);
  }
  printf("\n");
  return;
}

typedef struct{
int node;
int cost;
}data;

data heap[262144];
int hsize=0;

void resheap(){
    int i;
    heap[0].node=-1;
    heap[0].cost=-inf;
    for(i=1;i<262144;i++){
        heap[i].node=-1;
        heap[i].cost=inf;
    }
}

int swjud(data high,data low){
    if(high.cost > low.cost){return 1;}
    return -1;
}

void pqpush(data x){
    int a,b;
    data m;
    hsize++;
    heap[hsize]=x;
    a=hsize;
    while(1){
        b=a/2;
        if(swjud(heap[b],heap[a]) == 1){
            m=heap[a];heap[a]=heap[b];heap[b]=m;
        }
        else{break;}
        a=b;
    }
}

data pqpop(){
    int a,b,c;
    data m,sw;
    m=heap[1];
    heap[1]=heap[hsize];
    heap[hsize].node=-1;
    heap[hsize].cost=inf;
    hsize--;
    a=1;
    while(1){
        b=a*2;c=a*2+1;
        if(swjud(heap[a],heap[b])==-1 && swjud(heap[a],heap[c])==-1){break;}
        if(swjud(heap[b],heap[c])==-1){
            sw=heap[a];heap[a]=heap[b];heap[b]=sw;
            a=b;
        }
        else{
            sw=heap[a];heap[a]=heap[c];heap[c]=sw;
            a=c;
        }
    }
    return m;
}

//next_permutation
//https://qiita.com/Nikkely/items/0ddca51b3c0e60afbaab
int next_permutation(int a[],int n){
  int i=-1,j,k;
  for(k=n-2;k>=0;k--){
    if(a[k]<a[k+1]){i=k;break;}
  }
  if(i==-1){return -1;}
  for(k=n-1;k>=0;k--){
    if(a[i]<a[k]){j=k;break;}
  }
  swap(&a[i],&a[j]);
  for(k=i+1;k<n;k++){
    j=n-(k-i);
    if(k<j){swap(&a[j],&a[k]);}
    else{break;}
  }
  return 0;
}

int main(void){
  resheap();
  data pd,od;
  int i,j,n,m,k,a[524288],b,c,h,w,r=0,l,t,v;
  int lef[16];
  int perm[16]={0,1,2,3,4,5,6,7};
  int res[524288];
  scanf("%d",&n);
  for(i=1;i<=n;i++){
    scanf("%d",&a[i]);
    pd.node=i;
    pd.cost=i;
    pqpush(pd);
  }
  if(n==2){
    printf("-1\n");
  }
  else{
    v=llmin(n,8);
    i=0;t=-1;
    while(hsize>v){
      pd=pqpop();
      od=pqpop();
      if(t!=-1 && a[t]==pd.node){
        t=od.node;
        res[i]=t;
        pqpush(pd);
      }
      else{
        t=pd.node;
        res[i]=t;
        pqpush(od);
      }
      i++;
    }
    for(i=0;i<v;i++){
      od=pqpop();
      lef[i]=od.node;
      //printf("%d\n",lef[i]);
    }
    for(i=0;i<fact(v);i++){
      h=1;
      if(n==v){t=-1;}else{t=res[n-v];}
      for(j=0;j<v;j++){
        if(t!=-1 && a[t]==lef[perm[j]]){h=0;break;}
        t=lef[perm[j]];
        res[n-v+j]=t;
      }
      if(h==1){break;}
      next_permutation(perm,v);
    }
    prarr(res,n);
  }
  return 0;
}
