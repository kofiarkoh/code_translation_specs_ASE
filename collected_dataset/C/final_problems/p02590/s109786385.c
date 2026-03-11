#include <stdio.h>
#include <stdlib.h>

int main(int argc, char const *argv[]) {
  unsigned N, *A;
  unsigned long long sum = 0;

  scanf("%u", &N);

  A = malloc(sizeof(unsigned) * N);
  for (size_t i = 0; i < N; i++) {
    scanf("%u", &A[i]);
  }

  for (size_t i = 0; i < N; i++) {
    for (size_t j = i + 1; j < N; j++) {
      sum += (A[i] * A[j]) % 200003;
    }
  }

  printf("%llu\n", sum);

  return 0;
}
