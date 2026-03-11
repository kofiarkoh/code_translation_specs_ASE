#include <stdio.h>

int main(void) {
  int n;
  scanf("%d", &n);
  long a[200001] = {0};
  long b[200001] = {0};
  for(int i = 1;i <= n;i++) {
    scanf("%ld", a + i);
  }
  for(int i = 1;i < n;i++) {
    b[i-1] = a[i] ^ a[i+1];
}
  long x = 0;
  for(int i = 0;i < n;i += 2) {
    x ^= b[i];
  }
  for(int i = 1;i <= n;i++) {
    printf("%ld ", a[i] ^ x);
  }
  puts("");
}