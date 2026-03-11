#include<stdio.h>
#include<stdlib.h>
#include<string.h>

typedef struct binaryHeap{
  void *h;
  size_t heapSize;
  size_t maxSize;
  size_t size;
  int (*cmp)(const void *,const void *);
} heap;

heap* newHeap(const size_t size,int (*cmpF)(const void *,const void *)){
  const size_t maxSize=1;
  heap *res=(heap *)malloc(sizeof(heap));
  res->h=malloc(size*(maxSize+1));
  res->heapSize=0;
  res->maxSize=maxSize;
  res->size=size;
  res->cmp=cmpF;
  return res;
}

int isEmpty(const heap *h){
  return h->heapSize==0;
}

void freeHeap(heap *h){
  free(h->h);
  free(h);
  return;
}

void initHeap(heap *h){
  h->heapSize=0;
  return;
}

inline void heapFunc_swap(void * restrict a,void * restrict b,void * restrict tmp,size_t size){
  memcpy(tmp,a,size);
  memcpy(a,b,size);
  memcpy(b,tmp,size);
  return;
}

void push(heap *q,const void *p){
  if(q->heapSize==q->maxSize){
    q->h=realloc(q->h,q->size*(2*q->maxSize+1));
    q->maxSize*=2;
  }
  q->heapSize+=1;
  void *h=q->h;
  int k=q->heapSize;
  const size_t size=q->size;
  int (*cmp)(const void *,const void *)=q->cmp;
  memcpy(h+k*size,p,size);
  while(k>1){
    size_t parent=k/2;
    if(cmp(h+parent*size,h+k*size)<=0) return;
    heapFunc_swap(h+parent*size,h+k*size,h,size);
    k=parent;
  }
  return;
}

void pop(heap *q,void *res){
  void *h=q->h;
  const size_t size=q->size;
  if(res!=NULL) memcpy(res,h+size,size);
  memcpy(h+size,h+size*q->heapSize,size);
  q->heapSize-=1;
  int (*cmp)(const void *,const void *)=q->cmp;
  const size_t n=q->heapSize;
  size_t k=1;
  while(2*k+1<=n){
    size_t next=cmp(h+2*k*size,h+(2*k+1)*size)<=0?2*k:2*k+1;
    if(cmp(h+k*size,h+next*size)<=0) return;
    heapFunc_swap(h+k*size,h+next*size,h,size);
    k=next;
  }
  if(2*k<=n && cmp(h+k*size,h+2*k*size)>0) heapFunc_swap(h+k*size,h+2*k*size,h,size);
  return;
}

void top(const heap *q,void *res){
  memcpy(res,q->h+q->size,q->size);
  return;
}

int cmp(const void *a,const void *b){
  return *(int *)a-*(int *)b;
}

void run(void){
  int h,w,n;
  scanf("%d%d%d",&h,&w,&n);
  heap **q=(heap **)calloc(w+1,sizeof(heap *));
  int i;
  for(i=1;i<=w;i++) q[i]=newHeap(sizeof(int),cmp);
  for(i=0;i<n;i++){
    int x,y;
    scanf("%d%d",&x,&y);
    push(q[y],&x);
  }
  int now=1;
  int min=h;
  for(i=1;i<=w;i++){
    if(!isEmpty(q[i])){
      int v;
      pop(q[i],&v);
      while(!isEmpty(q[i]) && v<=now){
	pop(q[i],&v);
      }
      if(v>now && min>v-1){
	min=v-1;
      }
    }
    now++;
    if(i+1<=w && !isEmpty(q[i+1])){
      int v;
      pop(q[i+1],&v);
      while(!isEmpty(q[i+1]) && v<now){
	pop(q[i+1],&v);
      }
      while(!isEmpty(q[i+1]) && v==now){
	now++;
	pop(q[i+1],&v);
      }
      if(v>now){
	push(q[i+1],&v);
      }
    }
  }
  printf("%d\n",min);
}

int main(void){
  run();
  return 0;
}
