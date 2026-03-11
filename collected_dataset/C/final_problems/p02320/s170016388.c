#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

void run (void) {
  i32 n, lim;
  scanf ("%" SCNi32 "%" SCNi32, &n, &lim);
  i32 *dp = (i32 *) calloc (lim + 1, sizeof (i32));
  while (n--) {
    i32 v, w, m;
    scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &v, &w, &m);
    i32 k = 1;
    while (m > 0) {
      i32 a = v * k;
      i32 b = w * k;
      for (i32 i = lim; i >= b; --i) {
	if (dp[i] < dp[i - b] + a) {
	  dp[i] = dp[i - b] + a;
	}
      }
      m -= k;
      k *= 2;
      if (k > m) {
	k = m;
      }
    }
  }
  printf ("%" PRIi32 "\n", dp[lim]);
}

int main (void) {
  run ();
  return 0;
}

