#include<stdio.h>

int kill_all(long long explode, int n, long long* h, int a, int b){
  long long e = explode;
  for(int i = 0; i < n; i++){
    if(h[i] - explode * (long long)b > 0){
      e -= (h[i] - explode * (long long)b - 1ll) / (long long)(a - b) + 1ll;
    }
  }
  
  return e >= 0ll;
}

long long explosion(int n, long long* h, int a, int b){
  long long ok = (1ll << 60);
  long long ng = 0ll;
  while(ok - ng > 1ll){
    long long mid = (ok + ng) / 2ll;
    if(kill_all(mid, n, h, a, b)) ok = mid;
    else ng = mid;
  }
  
  return ok;
}

int main(){
  int N, A, B;
  scanf("%d %d %d", &N, &A, &B);
  
  long long h[N];
  for(int i = 0; i < N; i++) scanf("%lld", h + i);
  
  printf("%lld", explosion(N, h, A, B));
  
  return 0;
}