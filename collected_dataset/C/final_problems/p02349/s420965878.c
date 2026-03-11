#include<stdio.h>
#include<stdlib.h>

void add(int *bit,int index,int v){
  const int n=bit[0];
  for(int i=index;i<=n;i+=i&-i) bit[i]+=v;
}

int sum(int *bit,int index){
  int res=0;
  for(int i=index;i>0;i-=i&-i) res+=bit[i];
  return res;
}

void run(void){
  int n,q;
  scanf("%d%d",&n,&q);
  int *bit=(int *)calloc(n+1,sizeof(int));
  bit[0]=n;
  while(q--){
    int c;
    scanf("%d",&c);
    if(c==0){
      int l,r,v;
      scanf("%d%d%d",&l,&r,&v);
      add(bit,l,v);
      add(bit,r+1,-v);
    } else {
      int x;
      scanf("%d",&x);
      printf("%d\n",sum(bit,x));
    }
  }
}

int main(void){
  run();
  return 0;
}

