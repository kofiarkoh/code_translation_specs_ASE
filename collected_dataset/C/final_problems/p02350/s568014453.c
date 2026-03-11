#include<stdio.h>
#define INF 0x7fffffff
#define ST_SIZE ((1 << 18) - 1)
#define min(a, b) ((a) < (b) ? (a) : (b))
typedef long long int LL;
LL dat[ST_SIZE];
LL input(void);
void update(LL a, LL b, LL x, LL k, LL l, LL r);
LL find(LL a, LL b, LL k, LL l, LL r);
int main (void) {
  int command, x;
  LL i, n, q, s, t;
  n = input();
  q = input();
  for (i = 0; i < n; i++) dat[i] = INF;
  for (i = 0; i < q; i++) {
    command = input();
    if (!command) {
      s = input();
      t = input();
      x = input();
      update(s, t + 1, x, 0, 0, n);
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
void update(LL a, LL b, LL x, LL k, LL l, LL r)
{
  LL m = (l + r) / 2;
  if (a >= r || b <= l) return;
  else if (a <= l && b >= r) dat[k] = x;
  else {
    if (dat[k] != -1) {
      dat[k*2+1] = dat[k*2+2] = dat[k];
      dat[k] = -1;
    }
    update(a, b, x, k * 2 + 1, l ,m);
    update(a, b, x, k * 2 + 2, m, r);
  }
}
LL find(LL a, LL b, LL k, LL l, LL r)
{
  LL m = (l + r) / 2;
  if (a >= r || b <= l) return INF;
  else {
    if (dat[k] != -1) return dat[k];
    else {
      LL vl, vr;
      vl = find(a, b, k * 2 + 1, l, m);
      vr = find(a, b, k * 2 + 2, m, r);
      return min(vl, vr);
    }
  }
}
