#include <stdio.h>
#include <math.h>

#define NUM 100000

int prime[NUM + 1];

void init_prime() {
  int i, j;
  int sq_num = (int)sqrt((double)NUM);

  for (i = 0; i < NUM; i++) {
    prime[i] = i % 2;
  }
  prime[0] = prime[1] = 0;
  prime[2] = 1;

  for (i = 3; i < sq_num; i++) {
    if (prime[i] == 0) {
      continue;
    }
    for (j = i; i * j <= NUM; j += 2) {
      prime[i * j] = 0;
    }
  }
}

void solv(int q) {
  int i, j, c, l, r;

  for (i = 0; i < q; i++) {
    scanf("%d %d", &l, &r);
    c = 0;
    for (j = l; j <= r; j += 2) {
      if (prime[j] == 1 && prime[(j + 1) / 2] == 1) {
        c++;
      }
    }
    printf("%d\n", c);
  }
}

int main(int argc, char **argv) {
  int q;

  init_prime();
  while (scanf("%d", &q) != EOF) {
    solv(q);
  }

  return 0;
}
