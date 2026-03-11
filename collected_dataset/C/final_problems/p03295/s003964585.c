#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define df 0
#define MAX 100010

typedef struct{
  int a;
  int b;
}data;

int comp(const void* x,const void* y){
  data p=*((data*)x),q=*((data*)y);
  if(p.a!=q.a)return p.a-q.a;
  p.b-q.b;
}

data war[MAX];

int H[MAX]={};

void init_heap(int* heap);
void put_heap(int* heap, int x);
int pull_heap(int* heap);        // take min item
int flag_heap(int* heap, int itr, int temp);

int main(){
  int n,m;
  scanf("%d%d",&n,&m);
  int i;
  for(i=0;i<m;i++){
    scanf("%d%d",&((war[i]).a),&((war[i]).b));
  }
  qsort(war,m,sizeof(data),comp);

  int count=0;
  for(i=0;i<m;i++){
    data x=war[i];

    if(H[1]<=x.a){
      count++;
      init_heap(H);
    }
    put_heap(H,x.b);
  }
  
  printf("%d",count);
  return 0;
}


void init_heap(int* heap){
  heap[0]=0;
}


void put_heap(int* heap, int x){
  heap[0]++;
  int i=heap[0];
  while(i>1 && heap[i/2]>x){
    heap[i]=heap[i/2];
    i/=2;
  }
  heap[i]=x;
}


int pull_heap(int* heap){
  int x=heap[1],temp=heap[heap[0]],flag,itr=1,itr2;
  heap[0]--;
  flag=flag_heap(heap,itr,temp);
  while(flag){
    itr2=2*itr+(flag-1); //flag=1 ->left child, 2->right child.
    heap[itr]=heap[itr2];
    itr=itr2;
    flag=flag_heap(heap,itr,temp);
  }
  heap[itr]=temp;
  return x;
}

int flag_heap(int* heap, int itr,int temp){
  int a,flag;
  if(heap[0]<itr*2) flag=0;
  else if(heap[0]==itr*2)
    flag=(temp<heap[itr*2]?0:1);
  else if(heap[itr*2]<heap[itr*2+1])
    flag=(temp<heap[itr*2]?0:1);
  else flag=(temp<heap[itr*2+1]?0:2);

  //  flag= 0,1,or 2
  return flag;
}

/// confirm df==0 ///
