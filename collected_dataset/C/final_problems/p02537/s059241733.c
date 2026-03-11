#include <stdio.h>
#include <stdlib.h>

#define rep(i, n) for (int i = 0; i < n; i = i + 1)
#define array(N, t) (t*)calloc(N, sizeof(t))
#define ll long long
#define INF ((1LL << 62) - (1LL << 31))
#define max(p, q)((p) > (q) ? (p) : (q))
#define min(p, q)((p) < (q) ? (p) : (q))
#define swap(type, a, b) { type temp = a; a = b; b = temp; }

int N, K;
int * A;

int length(int i, int l) {
  if (i >= N) {
    return l;
  }
  int j = i;
  while(1) {
    if (j >= N) {
      break;
    }
    j += 1;
    if (abs(A[j] - A[i]) <= K) {
      break;
    }
  }
  return length(j, l + 1);
}

int main() {
  scanf("%d%d", &N, &K);

  A = array(N, int);

  rep(i, N) {
    scanf("%d", A + i);
  }

  // int dp[300010][300010] = {0};

  // int * dp;
  // dp = array(N, int);
  // rep(i, N + 1) {
  // dp[0][i] = 0;
  // }


  int ans = 0;
  rep(i, N) {
    ans = max(ans, length(i, 0));
  }

  printf("%d\n", ans);
  // printf("%d\n", dp[N]);

  free(A);
  // free(dp);

  return 0;
}
