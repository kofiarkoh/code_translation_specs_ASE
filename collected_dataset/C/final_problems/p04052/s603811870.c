#include<stdio.h>
#define swap(x,y) do {int a = x;x = y;y = a;} while(0)
int main (){
  int N,K;
  scanf("%d %d",&N,&K);
  int P[N];
  for(int i = 0;i < N;i ++){
    scanf("%d",&P[i]);
  }
  int nowmax = N;
  int minsoat[N];
  for(int i = 0;i < N;i ++){
    for(int j = 0;j < N;j ++){
      if(i + 1 == P[j]){
        minsoat[i] = j;
      }
    }
  }
  int whermax;
  int secondmax;
  for(int k = 0;k < N;k ++){
    while(nowmax > 1){
      whermax = minsoat[nowmax - 1];
      secondmax = minsoat[nowmax - 2];
      if(secondmax - whermax >= K){
        swap(P[secondmax],P[whermax]);
        swap(minsoat[nowmax -2],minsoat[nowmax-1]);
      }
      nowmax = nowmax - 1;
    }
    nowmax = N;
  }
  for(int i = 0;i < N;i ++){
    printf("%d\n",P[i]);
  }
  return 0;
}