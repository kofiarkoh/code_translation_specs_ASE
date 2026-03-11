#include<stdio.h>
#include<stdlib.h>
#include<string.h>

typedef struct pairingHeapNode{
  struct pairingHeapNode *next;
  struct pairingHeapNode *child;
  unsigned long long int val[];
} heapNode;

typedef struct pairingHeap{
  heapNode *root;
  int heapSize;
  size_t size;
  int (*cmp)(const void *,const void *);
} heap;

heapNode* newHeapNode(const void *v,const size_t size){
  heapNode *res=(heapNode *)malloc(sizeof(heapNode)+size);
  memcpy(res->val,v,size);
  res->next=NULL;
  res->child=NULL;
  return res;
}

void freeAllNode(heapNode *t){
  if(t==NULL) return;
  freeAllNode(t->next);
  freeAllNode(t->child);
  free(t);
  return;
}

heap* newHeap(const size_t size,int (*cmp)(const void *,const void *)){
  heap *res=(heap *)malloc(sizeof(heap));
  res->root=NULL;
  res->heapSize=0;
  res->size=size;
  res->cmp=cmp;
  return res;
}

void freeHeap(heap *h){
  freeAllNode(h->root);
  free(h);
  return;
}

int isEmpty(const heap *h){
  return h->heapSize==0;
}

heapNode* meld(heapNode *a,heapNode *b,int (*cmp)(const void *a,const void *b)){
  if(a==NULL) return b;
  if(b==NULL) return a;
  if(cmp(a->val,b->val)<=0){
    b->next=a->child;
    a->child=b;
    return a;
  } else {
    a->next=b->child;
    b->child=a;
    return b;
  }
}

void push(heap *h,const void *v){
  heapNode *p=newHeapNode(v,h->size);
  h->heapSize++;
  h->root=meld(h->root,p,h->cmp);
  return;
}

void pop(heap *h,void *v){
  if(v!=NULL) memcpy(v,h->root->val,h->size);
  heapNode *lst=h->root->child;
  int (*cmp)(const void *a,const void *b)=h->cmp;
  free(h->root);
  h->heapSize--;
  heapNode *r=NULL;
  while(lst!=NULL && lst->next!=NULL){
    heapNode *a=lst;
    heapNode *b=lst->next;
    heapNode *next=lst->next->next;
    a->next=b->next=NULL;
    r=meld(r,meld(a,b,cmp),cmp);
    lst=next;
  }
  if(lst!=NULL) r=meld(r,lst,cmp);
  h->root=r;
  return;
}

void top(const heap *h,void *v){
  memcpy(v,h->root->val,h->size);
  return;
}

typedef struct UnionFind{
  int *parent;
  int n;
} UnionFind;

UnionFind* newUnionFind(int n){
  UnionFind *u=(UnionFind *)calloc(1,sizeof(UnionFind));
  u->parent=(int *)calloc(n,sizeof(int));
  u->n=n;
  for(int i=0;i<n;i++) u->parent[i]=-1;
  return u;
}

int root(UnionFind *u,int x){
  if(u->parent[x]<0) return x;
  return u->parent[x]=root(u,u->parent[x]);
}

int same(UnionFind *u,int x,int y){
  return root(u,x)==root(u,y);
}

int getSize(UnionFind *u,int x){
  return -u->parent[root(u,x)];
}

void unite(UnionFind *u,int x,int y){
  x=root(u,x);
  y=root(u,y);
  if(x==y) return;
  if(u->parent[x]>u->parent[y]){
    int swap=x;
    x=y;
    y=swap;
  }
  u->parent[x]+=u->parent[y];
  u->parent[y]=x;
}

typedef long long int int64;

typedef struct edge{
  int a,b;
  int64 c;
} edge;

int cmpEdge(const void *a,const void *b){
  int64 p=((edge *)a)->c;
  int64 q=((edge *)b)->c;
  return p==q?0:p<q?-1:1;
}

void enumerate(int *a,int l,int r,int d,heap *h){
  if(r-l<=1) return;
  int m=(l+r)/2;
  enumerate(a,l,m,d,h);
  enumerate(a,m,r,d,h);
  int64 cost=a[l]+d;
  int index=l;
  int i;
  for(i=l+1;i<m;i++){
    if(cost>a[i]){
      cost=a[i];
      index=i;
    }
    cost+=d;
  }
  for(i=m;i<r;i++){
    push(h,&((edge){index,i,cost+a[i]}));
    cost+=d;
  }
  cost=a[r-1]+d;
  index=r-1;
  for(i=r-2;i>=m;i--){
    if(cost>a[i]){
      cost=a[i];
      index=i;
    }
    cost+=d;
  }
  for(i=m-1;i>=l;i--){
    push(h,&((edge){index,i,cost+a[i]}));
    cost+=d;
  }
}

void run(void){
  int n,d;
  scanf("%d%d",&n,&d);
  int *a=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  heap *h=newHeap(sizeof(edge),cmpEdge);
  enumerate(a,0,n,d,h);
  int64 ans=0;
  UnionFind *u=newUnionFind(n);
  int cnt=0;
  while(cnt<n-1){
    edge e;
    pop(h,&e);
    if(same(u,e.a,e.b)) continue;
    ans+=e.c;
    unite(u,e.a,e.b);
    cnt++;
  }
  printf("%lld\n",ans);
}

int main(void){
  run();
  return 0;
}
