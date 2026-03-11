#include <stdio.h>
#include <math.h>

int main(void) {

  long n;
  scanf("%ld", &n);
  long t_max[n],a_max[n];
  for (long i = 0; i < n; i++) {
    scanf("%ld", &t_max[i]);
  }
  for (long i = 0; i < n; i++) {
    scanf("%ld", &a_max[i]);
  }
  long mod = pow(10,9)+7;
  long t_min[n],a_min[n];
  for (long i = 0; i < n; i++) {
    t_min[i] = 1;
    if (i == 0) {
      t_min[i] = t_max[i];
    } else {
      if (t_max[i] != t_max[i-1]) {
        t_min[i] = t_max[i];
      }
    }
  }
  for (long i = n-1; i >= 0; i--) {
    a_min[i] = 1;
    if (i == n-1) {
      a_min[i] = a_max[i];
    } else {
      if (a_max[i] != a_max[i+1]) {
        a_min[i] = a_max[i];
      }
    }
  }
  long ans = 1;
  long min,max;
  for (long i = 0; i < n; i++) {
    if (a_max[i] < t_min[i] || t_max[i] < a_min[i]) {
      printf("0\n");
      return 0;
    }
    max = t_max[i];
    if (a_max[i] < max) {
      max = a_max[i];
    }
    min = t_min[i];
    if (a_min[i] > min) {
      min = a_min[i];
    }
    ans *= max-min+1;
    ans %= mod;
  }
  printf("%ld\n", ans);

  return 0;
}