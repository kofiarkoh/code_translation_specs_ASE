#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>
#include <limits.h>
#define Max(a, b) ((a) > (b) ? (a) : (b))
#define Min(a, b) ((a) > (b) ? (b) : (a))
#define abs(x) ((x) > 0 ? (x) : -(x))
#define MOD (ll)1000000007 //10^9 + 7
#define endl printf("\n")
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (n); i++)
#define rep2(i, n, m) for (int i = (n), i < (m); i++)

int
main(int argc, char *argv[])
{
  int t1, t2, r1, r2;
  scanf("%d %d %d %d", &t1, &t2, &r1, &r2);

  if (r1 == -1 || r2 == -1) {
    if (t1 < t2) printf("Alice\n");
    if (t2 < t1) printf("Bob\n");
    if (t1 == t2) printf("Draw\n");
  } else {
    if (r1 > r2) printf("Alice\n");
    if (r2 > r1) printf("Bob\n");
    if (r1 == r2) printf("Draw\n");
  }

  return 0;
}

