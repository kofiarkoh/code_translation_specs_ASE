#include<stdio.h>

int main(void) {
  long n;
  long a[300000];
  long i, j;
  long m = 1000000007;
  long sum = 0;
  
  scanf("%ld", &n);
  for (i = 0; i < n; i++) scanf("%ld", &a[i]);
  
  for (i = 0; i < n - 1; i++) {
    for (j = i + 1; j < n; j++) {
      sum = (sum + (a[i] ^ a[j])) % m;
    }
  }
  
  printf("%ld\n", sum);
  
  return 0;
}
