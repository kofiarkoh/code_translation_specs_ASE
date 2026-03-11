#include <stdio.h>
#include <stdint.h>

int main(void) {
  int64_t N, M;
  scanf("%ld %ld", &N, &M);

  if(N >= (M/2)) {
    printf("%ld\n", M/2);
  } else {
    int64_t REM = (M - 2 * N) / 4;
    printf("%ld\n", N+REM);
  }
  return 0;
}
