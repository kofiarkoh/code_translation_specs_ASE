#include<stdio.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

#define MIN(a,b) ((a)<(b)?(a):(b))

void run (void) {
  i32 n, a, b;
  scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &n, &a, &b);
  printf ("%" PRIi32 "\n", MIN (n * a, b));
}

int main (void) {
  run();
  return 0;
}