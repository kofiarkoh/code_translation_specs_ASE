#include<stdio.h>

int main() {
  long long n, k;
  long long a[200010];
  long long b[200010] = {};
  int i, j;
  scanf("%lld %lld", &n, &k);
  scanf("%lld", &a[1]);
  b[1] = a[1] - k;
  for (i = 2; i < n+1; i++) {
    scanf("%lld", &a[i]);
    b[i] = b[i-1] + a[i] - k;
  }
  /*printf("%lld\n", b[3]);*/
  int count = 0;
  for (i = 1; i < n+1; i++) {
    if (b[i] >= 0) {
      count += 1;
    }
    for (j = n; j > i; j--) {
      if (b[j] - b[i] >= 0) {
	count += 1;
      }
    }
  }
  printf("%d\n", count);
}
