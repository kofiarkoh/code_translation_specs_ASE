#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* subroutines */

int gcd(int p, int q) {
  int r, t;

  if (p < q) (t = p, p = q, q = t);
  if (q == 0) return p;
  while ((r = p % q) != 0) (p = q, q = r);
  return q;
}

int count_rec(int pi, int qi, int ai, int ni, int q0) {
  int a1, c, g, p1, q, q1;

  if (pi == 0) return 1;
  if (ni <= 0) return 0;

  c = 0;

  for (q = q0; q <= ai; q++) {
    p1 = pi * q - qi;
    if (p1 < 0) continue;
    q1 = qi * q;

    g = gcd(p1, q1);
    if (g > 1) {
      p1 /= g;
      q1 /= g;
    }

    a1 = ai / q;
    if (q1 <= a1)
      c += count_rec(p1, q1, a1, ni - 1, q);
  }

  return c;
}


/* main */

int main(int argc, char **argv) {
  int ai, c, ni, pi, qi;

  for (;;) {
    scanf("%d%d%d%d", &pi, &qi, &ai, &ni);
    if ((pi | qi | ai | ni) == 0) break;

    c = count_rec(pi, qi, ai, ni, 1);
    printf("%d\n", c);
  }

  return 0;
}