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
  int whermax;
  int secondmax;
  for(int k = 0;k < N;k ++){
    while(nowmax >= 2){
      for(int i = 0;i < N;i ++){
        if(P[i] == nowmax){
          whermax = i;
          i = N;
        }
      }
      for(int i = 0;i < N;i ++){
        if(P[i] == nowmax - 1){
          secondmax = i;
          i = N;
        }
      }
      if(secondmax - whermax >= K){
        swap(P[secondmax],P[whermax]);
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