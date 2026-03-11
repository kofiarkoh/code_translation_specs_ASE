#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  i32 *a = ALLOC (n, i32);
  for (i32 i = 0; i < n; ++i) {
    scanf ("%" SCNi32, a + i);
  }
  uint8_t *dp = ALLOC (k + 1, uint8_t);
  for (i32 i = 0; i < k; ++i) {
    if (dp[i]) continue;
    for (i32 j = 0; j < n && a[j] <= k - i; ++j) {
      dp[i + a[j]] = 1;
    }
  }
  puts (dp[k] ? "First" : "Second");
}

int main (void) {
  run();
  return 0;
}