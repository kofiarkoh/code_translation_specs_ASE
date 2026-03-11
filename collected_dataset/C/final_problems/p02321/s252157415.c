#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

typedef struct weightval{
  int64 w,v;
} node;

int cmp(const void *a,const void *b){
  node *x=(node *)a;
  node *y=(node *)b;
  if(x->w!=y->w) return x->w-y->w<0?-1:1;
  if(x->v!=y->v) return x->v-y->v<0?-1:1;
  return 0;
}

void enumerate(node *a,int64 *v,int64 *w,int n){
  a[0].v=0;
  a[0].w=0;
  int i;
  for(i=0;i<n;i++){
    int j;
    for(j=0;j<(1<<i);j++){
      int k=j+(1<<i);
      a[k].v=a[j].v+v[i];
      a[k].w=a[j].w+w[i];
    }
  }
  qsort(a,1<<n,sizeof(node),cmp);
  return;
}

void run(void){
  int n;
  int64 weight;
  scanf("%d%lld",&n,&weight);
  int64 v[40];
  int64 w[40];
  int i;
  for(i=0;i<n;i++) scanf("%lld%lld",v+i,w+i);
  const int alen=n/2;
  node *a=(node *)malloc(sizeof(node)*(1<<alen));
  const int blen=n-alen;
  node *b=(node *)malloc(sizeof(node)*(1<<blen));
  enumerate(a,v,w,alen);
  enumerate(b,v+alen,w+alen,blen);

  //printf("a\n");for(i=0;i<(1<<alen);i++) printf("%lld %lld\n",a[i].w,a[i].v);
  //printf("b\n");for(i=0;i<(1<<blen);i++) printf("%lld %lld\n",b[i].w,b[i].v);

  int64 *bmax=(int64 *)malloc(sizeof(int64)*(1<<blen));
  bmax[0]=b[0].v;
  for(i=1;i<(1<<blen);i++) bmax[i]=MAX(bmax[i-1],b[i].v);
  int64 max=0;
  for(i=0;i<(1<<alen);i++){
    if(a[i].w>weight) break;
    int l=0;
    int r=(1<<blen);
    while(l+1<r){
      int m=(l+r)/2;
      if(b[m].w<=weight-a[i].w){
	l=m;
      } else {
	r=m;
      }
    }
    max=MAX(max,a[i].v+bmax[l]);
  }
  printf("%lld\n",max);
  free(a);
  free(b);
  return;
}

int main(void){
  run();
  return 0;
}

