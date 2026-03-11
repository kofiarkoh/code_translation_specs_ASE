#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>
#define LL long long
#define INF 1<<31-1
#define MIN(x,y) ((x)<(y)?(x):(y))
#define MAX(x,y) ((x)<(y)?(y):(x))
#define FOR(i,a,n) for(i=a;i<n;i++)
#define MOD 1000000007
#define NUM 100005
void swap(int* a,int* b){int tmp=*a;*a=*b;*b=tmp;}
int cmp(const void* a,const void* b){return *(int*)a-*(int*)b;}

typedef struct{double s;int r,num;}P;

int cmpd(const void* a,const void* b){
	if(((P*)a)->s>((P*)b)->s)return -1;
  if(((P*)a)->s<((P*)b)->s)return 1;
  return 0;
}

int main() {
  int i,N,A[3],B[3],a,ans=0;
  scanf("%d%d%d%d%d%d%d",&N,&A[0],&A[1],&A[2],&B[0],&B[1],&B[2]);
  P p[6];
  FOR(i,0,3){
    p[i].s=(double)A[i]/(double)B[i];
    p[i].num=i;
    p[i+3].s=(double)B[i]/(double)A[i];
    p[i+3].num=i+3;
  }
  qsort(p,6,sizeof(P),cmpd);
  FOR(i,0,6){
    if(p[i].s>1)p[i].r=2;
    else if(p[i].s<1)p[i].r=0;
    else p[i].r=1;
  }
  //FOR(i,0,6)printf("%lf %d\n",p[i].s,p[i].num);
  //if(p[0].num>=3)ans+=(N*B[p[0].num-3])/A[p[0].num-3];
  //else ans+=(N*A[p[0].num])/B[p[0].num];
  FOR(i,0,3){
    if(p[i].r<=1)break;
    else{
      if(p[i].num>=3){
        ans+=(N/A[p[i].num-3])*B[p[i].num-3];
        N%=A[p[i].num-3];
      }
      else{
        ans+=(N/B[p[i].num])*A[p[i].num-3];
        N%=B[p[i].num];
      }
    }
  }
  ans+=N;

  printf("%d\n",ans);
	return 0;
}
