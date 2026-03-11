#include<stdio.h>
#include<stdlib.h>
#include<string.h>

#define MIN(a,b) ((a)<(b)?(a):(b))
#define MAX(a,b) ((a)>(b)?(a):(b))

typedef struct naiveVector{
  void *array;
  size_t size;
  int maxLen;
  int len;
} vector;

vector* newVector(const size_t size){
  vector *v=(vector *)malloc(sizeof(vector));
  v->array=NULL;
  v->size=size;
  v->maxLen=0;
  v->len=0;
  return v;
}

void freeVector(vector *v){
  free(v->array);
  free(v);
  return;
}

void push_back(vector *vec,const void *v){
  if(vec->len==vec->maxLen){
    vec->maxLen=vec->maxLen*2+1;
    vec->array=realloc(vec->array,vec->size*vec->maxLen);
  }
  memcpy((char *)vec->array+vec->size*vec->len,v,vec->size);
  vec->len++;
  return;
}

void erase_back(vector *vec){
  if(vec->len==0) return;
  vec->len--;
  return;
}

void* getAtRef(const vector *vec,const size_t index){
  return (char *)vec->array+vec->size*index;
}

void getAt(const vector *vec,const size_t index,void *res){
  memcpy(res,getAtRef(vec,index),vec->size);
}

void assignAt(vector *vec,const size_t index,const void *val){
  memcpy(getAtRef(vec,index),val,vec->size);
}

void run(void){
  int n;
  scanf("%d",&n);
  int *parent=(int *)calloc(n,sizeof(int));
  vector **child=(vector **)calloc(n,sizeof(vector));
  int i,j;
  for(i=0;i<n;i++) child[i]=newVector(sizeof(int));
  for(i=0;i<n-1;i++){
    int a,b;
    scanf("%d%d",&a,&b);
    a--;b--;
    if(a>b){
      int t=a;
      a=b;
      b=t;
    }
    parent[b]=a;
    push_back(child[a],&b);
  }
  int *min=(int *)calloc(n,sizeof(int));
  int *max=(int *)calloc(n,sizeof(int));
  const int ini=-10000000;
  for(i=0;i<n;i++){
    min[i]=ini;
    max[i]=-ini;
  }
  int k;
  scanf("%d",&k);
  for(i=0;i<k;i++){
    int v,p;
    scanf("%d%d",&v,&p);
    v--;
    min[v]=max[v]=p;
  }  
  for(i=n-1;i>0;i--){
    int u=parent[i];
    min[u]=MAX(min[u],min[i]-1);
    max[u]=MIN(max[u],max[i]+1);
  }
  for(i=0;i<n;i++){
    for(j=0;j<child[i]->len;j++){
      int u;
      getAt(child[i],j,&u);
      min[u]=MAX(min[u],min[i]-1);
      max[u]=MIN(max[u],max[i]+1);
    }
  }
  for(i=0;i<n;i++){
    if(max[i]<min[i]){
      printf("No\n");
      return;
    }
    if(i==0) continue;
    int u=parent[i];
    if(MAX(min[i],min[u])!=MIN(min[i],min[u])+1){
      printf("No\n");
      return;
    }
  }
  printf("Yes\n");
  for(i=0;i<n;i++){
    printf("%d\n",min[i]);
  }
  return;
}

int main(void){
  run();
  return 0;
}
