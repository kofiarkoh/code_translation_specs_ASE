#include <stdio.h>
#include <stdlib.h>

typedef long long int ll;

ll a[200000];

int cmp (const void *left, const void *right) {
  int l = *(int*)left;
  int r = *(int*)right;

  if (l < r) return -1;
  if (l > r) return +1;
  return 0;
}

ll dif (ll a, ll b) {
  return (a > b) ? (a - b) : (b - a);
}
ll smax (ll a, ll b) {
  return (a > b) ? a : b;
}

ll n;
ll solve() {
  qsort(a, n, sizeof(ll), cmp);

  ll result = dif(a[n - 1], a[0]);
  ll l = 0, r = n - 1;

  while (l + 1 < r - 1) {
    result += dif(a[l], a[r - 1]) + dif(a[l + 1], a[r]);
    l++;
    r--;
  }
  if (l + 1 == r - 1) {
    result += smax(dif(a[l], a[l + 1]), dif(a[r - 1], a[r]));
  }

  printf("%lld\n", result);
}

int main(void){
  ll i;
  scanf("%lld", &n);
  for (i = 0; i < n; i++) {
    scanf("%lld", &a[i]);
  }

  solve();
  return 0;
}
