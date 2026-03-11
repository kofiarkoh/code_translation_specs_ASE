#include<stdio.h>

const int mod = 1e9 + 7;

void swap(long long* a, long long* b){
  *a ^= *b;
  *b ^= *a;
  *a ^= *b;
}

long long inverse(long long x){
  long long m = mod, u = 1, v = 0;
  while(m){
    long long t = x / m;
    x -= t * m; swap(&x, &m);
    u -= t * v; swap(&u, &v);
  }
  u %= mod;
  if(u < 0) u += mod;
  return u;
}

int main(){
  long long H, W, A, B;
  scanf("%lld%lld%lld%lld", &H, &W, &A, &B);
  
  long long fact[H + W + 1];
  fact[0] = 1;
  for(int k = 1; k <= H + W; k++){
    fact[k] = fact[k - 1] * k % mod;
  }
  
  long long move = 0;
  for(long long j = B; j < W; j++){
    move += 
      fact[H - A - 1 + j] * inverse(fact[j] * fact[H - A - 1] % mod) % mod
      * fact[W - j + A - 2] % mod * inverse(fact[W - j - 1] * fact[A - 1] % mod) % mod;
    move %= mod;
  }
  
  printf("%lld\n", move);
  
  return 0;
}