#include <stdio.h>
#include <stdlib.h>

#define LMIN -10000000000
#define LMAX 100000000000

int main() {
  int a, b, q;
  scanf("%d%d%d", &a, &b, &q);
  long int s[a+2], t[b+2], x[q];
  s[0] = LMIN;
  t[0] = LMIN;
  s[a+1] = LMAX;
  t[b+1] = LMAX;
  for (size_t i = 1; i <= a; i++) {
    scanf("%ld", s+i);
  }
  for (size_t i = 1; i <= b; i++) {
    scanf("%ld", t+i);
  }
  for (size_t i = 0; i < q; i++) {
    scanf("%ld", x+i);
  }
  for (size_t i = 0; i < q; i++) {
    int la = 0, lb = 0, ua = a+1, ub = b+1, ma, mb;
    while (ua-la > 1) {
      ma = (ua+la)/2;
      if (s[ma] < x[i]) {
        la = ma;
      } else {
        ua = ma;
      }
    }
    while (ub-lb > 1) {
      mb = (ub+lb)/2;
      if (t[mb] < x[i]) {
        lb = mb;
      } else {
        ub = mb;
      }
    }
    long int ans = LMAX;
    for (size_t j = 0; j < 2; j++) {
      for (size_t k = 0; k < 2; k++) {
        long int d1, d2;
        d1 = labs(s[la+j] - x[i]) + labs(t[lb+k] - s[la+j]);
        d2 = labs(t[lb+k] - x[i]) + labs(t[lb+k] - s[la+j]);
        if (d1 < d2) {
          if (d1 < ans) {
            ans = d1;
          }
        } else {
          if (d2 < ans) {
            ans = d2;
          }
        }
      }
    }
    printf("%ld\n", ans);
  }
  return 0;
}
