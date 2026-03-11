#include <stdio.h>

int A[112345];

int main(void){
  
  int N;
  scanf("%d", &N);
  
  int i;
  for( i = 0; i < 100001; i++ ){
    A[i] = 0;
  }
  
  int cnt = 0;
  int num;
  
  for( i = 0; i < N; i++ ){
    scanf("%d", &num);
    A[num]++;
    if( A[num] > 1 ) cnt++;
  }
  
  int ans;
  
  if( cnt % 2 ) ans = N-cnt-1;
  else{         ans = N-cnt;}
  
  printf("%d\n", ans);
  
  return 0;
}