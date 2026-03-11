#include <stdio.h>
#include <stdlib.h>

int main(void){

  long n;
  scanf("%ld", &n);
  long a[n];
  for (long i = 0; i < n; i++) {
    scanf("%ld", &a[i]);
  }
  long sum[n];
  sum[0] = a[0];
  for (long i = 1; i < n; i++) {
    sum[i] = sum[i-1]+a[i];
  }
  long all = sum[n-1];
  long other = all-sum[0];
  long min = labs(other-sum[0]);
  for (long i = 1; i < n-1; i++) {
    other = all-sum[i];
    if (labs(other-sum[i]) < min) {
      min = labs(other-sum[i]);
    }
  }
  printf("%ld\n", min);

  return 0;
}