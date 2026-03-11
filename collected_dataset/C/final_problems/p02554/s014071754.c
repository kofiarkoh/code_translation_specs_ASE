#include<stdio.h>

unsigned long power(unsigned long a, unsigned long e, unsigned long m) {
  unsigned long s = 1;
  while(e>0) {
    if(e % 2 == 1) s = s * a % m;
    a = a * a % m;
    e = e / 2 % m;
  }
  return s;
}

int main(void) {
  unsigned long n;
  scanf("%lu", &n);
  printf("%lu\n", (1000000007 + power(10, n, 1000000007) + power(8, n, 1000000007) - power(9, n, 1000000007)*2) % 1000000007);
  return 0;
}
