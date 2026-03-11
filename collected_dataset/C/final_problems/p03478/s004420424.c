#include <stdio.h>

int main(void){
  int N,A,B;
  int ans = 0;

  scanf("%d%d%d",&N,&A,&B);

  for(int i = 1;i <= N;i++){
    int j = i;
    int sum = 0;
    while(j > 0){
      sum += j%10;
      j /= 10;
    }
    if(A <= sum && sum <= B){
      ans += i;
    }
  }

  printf("%d\n",ans);
  return 0;
}