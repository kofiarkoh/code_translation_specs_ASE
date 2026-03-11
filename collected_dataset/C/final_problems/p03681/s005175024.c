#include <stdio.h>
#define abs(x) ((x)<0 ? -(x) : (x))
#define max(x, y) ((x)<(y) ? (y) : (x))
#define min(x, y) ((x)>(y) ? (y) : (x))

int main(){
  long n, m;  scanf("%ld %ld", &n, &m);

  if(abs(n-m) > 1){
    printf("0\n");
    return 0;
  }
  long mod = 1e9 + 7;
  long table[100005] = {0};
  table[1] = 1;
  for(long i=2; i<=max(n, m); i++)
    table[i] = (table[i-1] * i) % mod;

  long ans = (table[m] * table[n]) % mod;
  if(n == m)  ans = (ans * 2) % mod;

  printf("%ld\n", ans);

  return 0;
}
