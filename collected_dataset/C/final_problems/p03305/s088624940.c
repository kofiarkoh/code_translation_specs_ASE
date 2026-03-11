#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

typedef struct dnode{
  int v;
  int64 val;
} node;

int cmp(node *a,node *b){
  return a->val<=b->val?1:0;
}

void swap(node *a,node *b){
  node t=*a;
  *a=*b;
  *b=t;
  return;
}

void push(node *h,int v,int64 val){
  h[++h[0].v].v=v;
  h[h[0].v].val=val;
  int k=h[0].v;
  while(k>1){
    if(cmp(h+k/2,h+k)) break;
    swap(h+k/2,h+k);
    k/=2;
  }
  return;
}

int pop(node *h){
  int res=h[1].v;
  h[1]=h[h[0].v--];
  int k=1;
  while(2*k+1<=h[0].v){
    if(cmp(h+k,h+2*k) && cmp(h+k,h+2*k+1)) break;
    int next=cmp(h+2*k,h+2*k+1)?2*k:2*k+1;
    swap(h+k,h+next);
    k=next;
  }
  if(2*k<=h[0].v && !cmp(h+k,h+2*k)) swap(h+k,h+2*k);
  return res;
}

void dijkstra(int from,int64 *array,int *start,int *vertex,int *cost,int *next,int n){
  int *used=(int *)malloc(sizeof(int)*(n+1));
  int i;
  for(i=0;i<=n;i++){
    array[i]=10000000000000000LL;
    used[i]=0;
  }
  node *h=(node *)malloc(sizeof(node)*100001);
  h[0].v=0;
  array[from]=0;
  push(h,from,0);
  while(h[0].v>0){
    int v=pop(h);
    if(used[v]) continue;
    used[v]=1;
    int p=start[v];
    while(p>0){
      int u=vertex[p];
      if(array[v]+cost[p]<array[u]){
	array[u]=array[v]+cost[p];
	push(h,u,array[u]);
      }
      p=next[p];
    }
  }
  free(used);
  free(h);
  return;
}

#define ADD(i,u,v,a,b) do{vertex[i]=v;costA[i]=a;costB[i]=b;next[i]=start[u];start[u]=i;}while(0)

void run(void){
  int n,m,s,t;
  scanf("%d%d%d%d",&n,&m,&s,&t);
  int i;
  int *start=(int *)malloc(sizeof(int)*(n+1));
  int *vertex=(int *)malloc(sizeof(int)*(2*m+1));
  int *costA=(int *)malloc(sizeof(int)*(2*m+1));
  int *costB=(int *)malloc(sizeof(int)*(2*m+1));
  int *next=(int *)malloc(sizeof(int)*(2*m+1));
  next[0]=0;
  for(i=0;i<=n;i++) start[i]=0;
  for(i=1;i<=m;i++){
    int u,v,a,b;
    scanf("%d%d%d%d",&u,&v,&a,&b);
    ADD(2*i-1,u,v,a,b);
    ADD(2*i,v,u,a,b);
  }
  int64 *sTo=(int64 *)malloc(sizeof(int64)*(n+1));
  dijkstra(s,sTo,start,vertex,costA,next,n);
  int64 *tTo=(int64 *)malloc(sizeof(int64)*(n+1));
  dijkstra(t,tTo,start,vertex,costB,next,n);
  int64 *min=(int64 *)malloc(sizeof(int64)*(n+1));
  min[n]=sTo[n]+tTo[n];
  for(i=n-1;i>=1;i--){
    min[i]=MIN(min[i+1],sTo[i]+tTo[i]);
  }
  const int64 mm=1000000000000000LL;
  for(i=1;i<=n;i++){
    printf("%lld\n",mm-min[i]);
  }
}

int main(void){
  run();
  return 0;
}
