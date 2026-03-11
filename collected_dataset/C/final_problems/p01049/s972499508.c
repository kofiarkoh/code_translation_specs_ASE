#include<stdio.h>

int main(){
  int N;
  int a,d;
  int M;
  int K;
  int i;
  int t;
  int xyz[11][3];
  scanf("%d",&N);
  scanf("%d %d",&a,&d);
  scanf("%d",&M);
  int A[N];
  for(i=0;i<N;i++){
    A[i]=a+d*i;
  }
  for(i=0;i<M;i++){
    scanf("%d%d%d",&xyz[i][0],&xyz[i][1],&xyz[i][2]);
    if(xyz[i][1]==0){
      t=A[xyz[i][1]-1];
      A[xyz[i][1]-1]=A[xyz[i][2]];
      A[xyz[i][2]-1]+t;
    }else{
      A[xyz[i][1]-1]=A[xyz[i][2]-1];
    }
  }
  scanf("%d",&K);
  printf("%d\n",A[K-1]);

  return 0;
}