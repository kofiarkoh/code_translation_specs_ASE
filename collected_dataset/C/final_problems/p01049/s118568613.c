#include<stdio.h>

int main(){
  int N;
  int a,d;
  int M;
  int K;
  int i;
  int t;
  int x,y,z;
  scanf("%d",&N);
  scanf("%d %d",&a,&d);
  scanf("%d",&M);
  int A[N];
  for(i=0;i<N;i++){
    A[i]=a+d*i;
  }
  for(i=0;i<M;i++){
    scanf("%d%d%d",&x,&y,&z);
    if(x==0){
      t=A[y-1];
      A[y-1]=A[z-1];
      A[z-1]+t;
    }else{
      A[y-1]=A[z-1];
    }
  }
  scanf("%d",&K);
  printf("%d\n",A[K-1]);

  return 0;
}