#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>

typedef int32_t i32;

typedef struct RMQandRAQSegmentTree{
  i32 *add;
  i32 *val;
  i32 size,bit;
} segTree;

segTree* newRMQandRAQ(const i32 n){
  i32 k=0;
  while((1<<k)<n) k++;
  i32 size=1<<k;
  segTree *s=(segTree *)malloc(sizeof(segTree));
  s->add=(i32 *)calloc(2*size,sizeof(i32));
  s->val=(i32 *)calloc(2*size,sizeof(i32));
  s->size=size;
  s->bit=k;
  return s;
}

i32 getVal(segTree *s,i32 k){
  return s->val[k]+s->add[k];
}

i32 func(i32 a,i32 b){
  return a<b?a:b;
}

void downTree(segTree *s,i32 x){
  x+=s->size;
  for(i32 i=s->bit;i>0;i--){
    i32 k=x>>i;
    s->add[2*k  ]+=s->add[k];
    s->add[2*k+1]+=s->add[k];
    s->val[k]=func(getVal(s,2*k),getVal(s,2*k+1));
    s->add[k]=0;
  }
}

void upTree(segTree *s,i32 x){
  for(x=(x+s->size)>>1;x>0;x>>=1){
    s->val[x]=func(getVal(s,2*x),getVal(s,2*x+1));
  }
}

void add(segTree *s,i32 l,i32 r,i32 v){
  downTree(s,l);
  downTree(s,r-1);
  for(i32 x=l+s->size,y=r+s->size;x<y;x>>=1,y>>=1){
    if(x&1) s->add[x++]+=v;
    if(y&1) s->add[--y]+=v;
  }
  upTree(s,l);
  upTree(s,r-1);
}

int find(segTree *s,int l,int r){
  downTree(s,l);
  downTree(s,r-1);
  i32 index[32];
  i32 len=0;
  for(l+=s->size,r+=s->size;l<r;l>>=1,r>>=1){
    if(l&1) index[len++]=l++;
    if(r&1) index[len++]=--r;
  }
  i32 res=getVal(s,index[--len]);
  while(len>0) res=func(res,getVal(s,index[--len]));
  return res;
}

void run(void){
  int n,q;
  scanf("%d%d",&n,&q);
  segTree *s=newRMQandRAQ(n);
  while(q--){
    int t,l,r;
    scanf("%d%d%d",&t,&l,&r);
    if(t==0){
      int v;
      scanf("%d",&v);
      add(s,l,r+1,v);
    } else {
      printf("%d\n",find(s,l,r+1));
    }
  }
}

int main(void){
  run();
  return 0;
}

