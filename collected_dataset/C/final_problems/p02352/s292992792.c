#include<stdio.h>
#include<limits.h>
#define ST_SIZE ((1 << 18) - 1)
#define min(a,b) ((a) < (b) ? (a) : (b))
typedef long long int LL;
LL segadd[ST_SIZE], segmin[ST_SIZE];
LL input(void);
void add(LL a, LL b, int x, LL k, LL l, LL r);
LL find(LL a, LL b, LL k, LL l, LL r);
int main (void) {
  int command, x;
  LL i, n, q, s, t;
  n = input();
  q = input();
  for (i = 0; i < q; i++) {
    command = input();
    if (!command) {
      s = input();
      t = input();
      x = input();
      add(s, t + 1, x, 0, 0, n);
    } else {
      s = input();
      t = input();
      printf("%lld\n", find(s, t + 1, 0, 0, n));
    }
  }
}
LL input(void)
{
  LL n = 0;
  int c = getchar_unlocked();
  if (c == '-') {
    c = getchar_unlocked();
    do {
      n = 10 * n + c - '0';
      c = getchar_unlocked();
    } while (c >= '0' && c <= '9');
    return -n;
  } else {
    do {
      n = 10 * n + c - '0';
      c = getchar_unlocked();
    } while (c >= '0' && c <= '9');
    return n;
  }
}
void add(LL a, LL b, int x, LL k, LL l, LL r)
{
  if (a >= r || b <= l) return;
  else if (a <= l && b >= r) segadd[k] += x;
  else {
    add(a, b, x, k * 2 + 1, l, (l + r) / 2);
    add(a, b, x, k * 2 + 2 , (l + r) / 2, r);
    segmin[k] = min(segmin[k*2+1] + segadd[k*2+1], segmin[k*2+2] + segadd[k*2+2]);
  }
}
LL find(LL a, LL b, LL k, LL l, LL r)
{
  if (a >= r || b <= l) return INT_MAX;
  else if (a <= l && b >= r) return (segmin[k] + segadd[k]);
  else {
    LL vl, vr;
    vl = find(a, b, k * 2 + 1, l, (l + r) / 2);
    vr = find(a, b, k * 2 + 2 , (l + r) / 2, r);
    return (min(vl, vr) + segadd[k]);
  }
}
