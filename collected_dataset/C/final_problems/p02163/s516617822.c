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
  int n;
  scanf("%d", &n);
  ll x = 1, y = 0;
  rep(i, n) {
    int q; ll X;
    scanf("%d %lld", &q, &X);
    if (q == 1) {
      x *= X; y *= X;
    } else if (q == 2) {
      y += X;
    } else {
      y -= X;
    }
  }

  printf("%lld %lld\n", -y, x);

  return 0;
}

