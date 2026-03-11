//tree DP
//ABC036-D
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

typedef struct{
int val;
int node;
}sd;

int sdsortfnc(const void *a,const void *b){
if(((sd*)a)->val > ((sd*)b)->val){return -1;}
if(((sd*)a)->val < ((sd*)b)->val){return 1;}
return 0;
}

typedef struct{
    long long st;
    long long fi;
    long long kr;
}rs;

typedef struct{
    long long st;
    long long kz;
}mkj;

int sortfnc(const void *a,const void *b){
if(((rs*)a)->st == ((rs*)b)->st){return 0;}
if(((rs*)a)->st < ((rs*)b)->st){return -1;}
return 1;
}

void makemkj(rs g[],mkj x[],long long n){
    long long i,ms=0,nst=g[0].st;
    for(i=1;i<n;i++){
        if(g[i].st!=g[i-1].st){
            x[nst].kz=i-ms;
            x[nst].st=ms;
            nst=g[i].st;ms=i;
        }
    }
    x[nst].kz=n-ms;
    x[nst].st=ms;
}

long long dist[4096],par[4096];
void dfs(long long t,long long l,long long bp,rs g[],mkj x[]){
  long long i;
  if(dist[t]<=l){return;}
  dist[t]=l;
  par[t]=bp;
  for(i=x[t].st;i<x[t].st+x[t].kz;i++){
    dfs(g[i].fi,l+1,t,g,x);
  }
}

int main(void){
    long long i,j,n,m,k,a,b,c,h,w,r=0,l,t;
    long long arr[4096],mem[4096]={0};
    rs g[4096];
    mkj x[4096];
    sd dat[4096];
    bool dp[8192];
    scanf("%lld",&n);
    for(i=0;i<(n-1);i++){
      scanf("%lld",&a);
      g[2*i].st=a;
      g[2*i].fi=i+2;
      g[2*i].kr=1;
      g[2*i+1].st=i+2;
      g[2*i+1].fi=a;
      g[2*i+1].kr=1;
    }
    for(i=1;i<=n;i++){
      scanf("%lld",&arr[i]);
    }
    qsort(g,2*(n-1),sizeof(g[0]),sortfnc);
    makemkj(g,x,2*(n-1));
    for(i=0;i<=n;i++){
      dist[i]=inf;
    }
    dfs(1,0,-1,g,x);
    for(i=0;i<n;i++){
      dat[i].node=i+1;
      dat[i].val=dist[i+1];
    }
    qsort(dat,n,sizeof(dat[0]),sdsortfnc);
    for(i=0;i<n;i++){
      w=dat[i].node;
      for(j=1;j<=arr[w];j++){
        dp[j]=0;
      }
      dp[0]=1;
      t=0;
      for(j=x[w].st;j<x[w].st+x[w].kz;j++){
        if(par[w]==g[j].fi){continue;}
        //printf("%lld -> %lld:<%lld %lld>\n",w,g[j].fi,arr[g[j].fi],mem[g[j].fi]);
        t+=(arr[g[j].fi]+mem[g[j].fi]);
        for(k=arr[w];k>=0;k--){
          if(dp[k]==0){continue;}
          dp[k]=0;
          if(k+arr[g[j].fi]<=arr[w]){dp[k+arr[g[j].fi]]=1;}
          if(k+mem[g[j].fi]<=arr[w]){dp[k+mem[g[j].fi]]=1;}
        }
      }
      for(j=arr[w];j>=0;j--){
        if(dp[j]==1){
          mem[w]=t-j;
          //printf("%lld:%lld\n",w,mem[w]);
          break;
        }
        if(j==0){
          printf("IMPOSSIBLE\n");
          return 0;
        }
      }
    }
    printf("POSSIBLE\n");
    return 0;
}
