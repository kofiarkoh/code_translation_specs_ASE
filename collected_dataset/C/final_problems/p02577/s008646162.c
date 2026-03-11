#include <stdio.h>
#include <stdlib.h>

#define rep(i, n) for (int i = 0; i < n; i = i + 1)
#define array(N, t) (t*)calloc(N, sizeof(t))
#define ll long long
#define INF ((1LL << 62) - (1LL << 31))
#define max(p, q)((p) > (q) ? (p) : (q))
#define min(p, q)((p) < (q) ? (p) : (q))

int main() {
  char N[20010];
  scanf("%s", N);

  ll sum = 0;
  rep(i, 20010) {
    if (N[i] == '\0') {
      break;
    }
    sum += (N[i] - '0');
  }

  if (sum % 9 == 0) {
    printf("Yes\n");
  } else {
    printf("No\n");
  }

  return 0;
}
