#include <stdio.h>
#include <stdlib.h>

#define SWAP(T,X,Y) {const T gensym1 = X; X = Y; Y = gensym1;}

int main(void) {
  unsigned long long int n, x;
  unsigned long long int res;

  scanf("%llu%llu", &n, &x);

  res = n;
  n = n - x;
  for (;;) {
    if (n % x == 0) {
      res += 2 * n - x;
      break ;
    } else {
      res += x * 2 * (n / x);
      n %= x;
      SWAP(unsigned long long int, n, x);
    }
  }
  printf("%llu\n", res);
  
  return (0);
}
