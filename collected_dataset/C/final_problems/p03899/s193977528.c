#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))

typedef struct indexVal{
  int64 val;
  int index;
} node;

typedef struct deque{
  node *a;
  int front;
  int last;
  int len;
} deque;

deque* newDeque(int len){
  deque *d=(deque *)calloc(1,sizeof(deque));
  d->a=(node *)calloc(len,sizeof(node));
  d->front=d->last=0;
  d->len=len;
  return d;
}

void push_back(deque *d,node v){
  d->a[d->last++]=v;
  d->last%=d->len;
}

node pop_front(deque *d){
  node t=d->a[d->front++];
  d->front%=d->len;
  return t;
}

node pop_back(deque *d){
  d->last=(d->last+d->len-1)%d->len;
  return d->a[d->last];
}

node get_front(deque *d){
  return d->a[d->front];
}

node get_back(deque *d){
  return d->a[(d->last+d->len-1)%d->len];
}

int isEmpty(deque *d){
  return d->front==d->last;
}

#define POS(i,j) ((i)*(k+1)+(j))

void run(void){
  int n,m,k;
  scanf("%d%d%d",&n,&m,&k);
  int *a=(int *)calloc(n+1,sizeof(int));
  int i;
  for(i=1;i<=n;i++) scanf("%d",a+i);
  int64 *dp=(int64 *)calloc((k+1)*(n+1),sizeof(int64));
  deque **deq=(deque **)calloc(k,sizeof(deque *));
  for(i=0;i<k;i++) deq[i]=newDeque(m+1);
  push_back(deq[0],(node){0,n});
  for(i=1;i<=n;i++){
    for(int j=MIN(k,i);j>=1;j--){
      node t=get_front(deq[j-1]);
      while(t.index<i-m){
	pop_front(deq[j-1]);
	t=get_front(deq[j-1]);
      }
      dp[POS(i,j)]=(int64)j*a[i]+t.val;//l=1..m
      if(j<k){
	while(!isEmpty(deq[j])){
	  t=get_back(deq[j]);
	  if(t.val>dp[POS(i,j)]) break;
	  pop_back(deq[j]);
	}
	push_back(deq[j],(node){dp[POS(i,j)],i});
      }
    }
  }
  int64 ans=0;
  for(i=m;i<=n;i++) ans=MAX(ans,dp[POS(i,k)]);
  printf("%lld\n",ans);
}

int main(void){
  run();
  return 0;
}
