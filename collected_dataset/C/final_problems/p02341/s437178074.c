#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  i32 ans = n <= k ? 1 : 0;
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}

