#include<stdio.h>

int main(void){

  int N=0, K=0;//ボールの数,ペンキの色の種類の数
  int sum=0;
  int i=0;

  scanf("%d %d", &N, &K);

  for(i=1; i<=N; i++){
    if(i==1)
      sum=K;
    else{
      sum=sum*(K-1);
    }
  }

  printf("%d", sum);

  return 0;

}
