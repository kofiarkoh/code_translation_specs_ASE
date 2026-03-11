#include<stdio.h>

int main(){
  int N;
  scanf("%d", &N);
  
  int A[N];
  for(int i = 0; i < N; i++) scanf("%d", A + i);
  
  long long cnt = 0;
  int now = -1;
  for(int i = 0; i < N; i++){
    if(A[i] == now + 1){
      now++;
    }else if(A[i] <= now){
      cnt += now;
      now = A[i];
    }else{
      cnt = -1;
      break;
    }
  }
  if(A[N - 1] == now) cnt += now;
  
  printf("%lld", cnt);
  
  return 0;
}