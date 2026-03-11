#include<stdio.h>
#include<stdlib.h>

typedef struct {
  int val;
  int index;
} node;

node min(node a,node b){
  return a.val<b.val?a:b;
}

typedef struct RMQ{
  node *a;
  int size;
} segTree;

segTree* newRMQ(const int n,const int *h){
  int k=1;
  while(k<n) k*=2;
  segTree *s=(segTree *)malloc(sizeof(segTree));
  s->a=(node *)calloc(2*k,sizeof(node));
  s->size=k;
  int i;
  for(i=0;i<n;i++){
    s->a[i+k]=(node){h[i],i};
  }
  for(;i<k;i++){
    s->a[i+k]=(node){1000000000+1,i};
  }
  for(i=k-1;i>=1;i--) s->a[i]=min(s->a[2*i],s->a[2*i+1]);
  return s;
}

node query(segTree *s,int l,int r){
  node res={1000000000+1,-1};
  for(l+=s->size,r+=s->size;l<r;l>>=1,r>>=1){
    if(l&1) res=min(res,s->a[l++]);
    if(r&1) res=min(res,s->a[--r]);
  }
  return res;
}

typedef struct {
  int l,r;
} range;

typedef long long int int64;

void run(void){
  int n;
  scanf("%d",&n);
  int *h=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",h+i);
  segTree *s=newRMQ(n,h);
  range stack[64];
  int top=0;
  stack[top++]=(range){0,n};
  int64 max=0;
  while(top>0){
    range t=stack[--top];
    if(t.l>=t.r) continue;
    node q=query(s,t.l,t.r);
    int64 area=(int64)q.val*(t.r-t.l);
    if(max<area) max=area;
    int m=q.index;
    if(m-t.l<=t.r-(m+1)){
      stack[top++]=(range){m+1,t.r};
      stack[top++]=(range){t.l,m};
    } else {
      stack[top++]=(range){t.l,m};
      stack[top++]=(range){m+1,t.r};
    }
  }
  printf("%lld\n",max);
}

int main(void){
  run();
  return 0;
}

