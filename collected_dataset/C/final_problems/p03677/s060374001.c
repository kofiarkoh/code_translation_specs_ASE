#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

void run (void) {
  i32 n, m;
  scanf ("%" SCNi32 "%" SCNi32, &n, &m);
  i32 *a = (i32 *) calloc (n, sizeof (i32));
  for (i32 i = 0; i < n; ++i) {
    scanf ("%" SCNi32, a + i);
    a[i]--;
  }
  i64 sum = 0;
  i64 *imos = (i64 *) calloc (2 * m, sizeof (i64));
  i64 *ans = (i64 *) calloc (2 * m, sizeof (i64));
  for (i32 i = 1; i < n; ++i) {
    if (a[i] > a[i - 1]) {
      sum += a[i] - a[i - 1];
      imos[a[i - 1] + 2]++;
      imos[a[i] + 1]--;
      ans[a[i] + 1] -= a[i] - a[i - 1] - 1;
    } else {
      sum += a[i] + m - a[i - 1];
      imos[a[i - 1] + 2]++;
      imos[a[i] + m + 1]--;
      ans[a[i] + m + 1] -= a[i] + m - a[i - 1] - 1;
    }
  }
  for (i32 i = 1; i < 2 * m; ++i) {
    imos[i] += imos[i - 1];
  }
  for (i32 i = 1; i < 2 * m; ++i) {
    ans[i] += ans[i - 1] + imos[i];
  }
  for (i32 i = m; i < 2 * m; ++i) {
    ans[i - m] += ans[i];
  }
  i64 min = sum;
  for (i32 i = 0; i < m; ++i) {
    i64 v = sum - ans[i];
    if (v < min) min = v;
  }
  printf ("%" PRIi64 "\n", min);
}

int main (void) {
  run();
  return 0;
}
