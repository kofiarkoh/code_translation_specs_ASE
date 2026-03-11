#include<stdio.h>

int decreasable(long long K, long long* a, int N){
  long long count = 0;
  for(int i = 0; i < N; i++){
    count += (a[i] + K + 1) / (N + 1);
  }
  
  return count <= K;
}

int main(){
  int N; scanf("%d", &N);
  
  long long a[N];
  for(int i = 0; i < N; i++) scanf("%lld", a + i);
  
  long long ok = 10e17;
  long long ng = -1;
  while(ok - ng > 1){
    long long K = (ok + ng) / 2;
    if(decreasable(K, a, N)) ok = K;
    else ng = K;
  }
  
  for(int i = 1; i <= ok && i <= N * N; i++){
    if(decreasable(ok - i, a, N)) ng = ok - i - 1;
  }
  ok = ng + 1;
  
  printf("%lld\n", ok);
  
  return 0;
}