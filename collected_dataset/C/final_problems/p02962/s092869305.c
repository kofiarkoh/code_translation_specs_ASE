#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>
#include<math.h>

typedef int32_t i32;
typedef int64_t i64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))
#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))
#define SORT(a,num,cmp) qsort((a),(num),sizeof(*(a)),cmp)

#define F 4

const i32 mod[F] = {1000001011,1000001021,1000001053,1000001087};
const i32 rad = 100000;

i32 mod_pow (i32 r, i32 n, i32 mod) {
  i64 t = 1;
  i64 s = r;
  while (n > 0) {
    if (n & 1) t = t * s % mod;
    s = s * s % mod;
    n >>= 1;
  }
  return t;
}

void query (i32 **dp, i32 l, i32 r, i32 *res) {
  for (i32 i = 0; i < F; ++i) {
    res[i] = (dp[l][i] - (i64) dp[r][i] * mod_pow (rad, r - l, mod[i]) % mod[i] + mod[i]) % mod[i];
  }
}

i32 equal (i32 *a, i32 *b) {
  for (i32 i = 0; i < F; ++i) {
    if (a[i] != b[i]) {
      return 0;
    }
  }
  return 1;
}

void run (void) {
  const i32 LEN = 500000;
  char *s = ALLOC (3 * LEN + 1, char);
  char *t = ALLOC (LEN + 1, char);
  scanf ("%s%s", s, t);
  i32 n = strlen (s);
  i32 m = strlen (t);
  const i32 d = n;
  while (n < 2 * m || n < 2 * d) {
    memcpy (s + n, s, sizeof (char) * n);
    n += d;
  }
  i32 **hash = ALLOC (n + 1, i32 *);
  {
    i32 *buf = ALLOC (F * (n + 1), i32);
    for (i32 i = 0; i <= n; ++i) {
      hash[i] = buf;
      buf += F;
    }
  }
  for (i32 i = n - 1; i >= 0; --i) {
    for (i32 j = 0; j < F; ++j) {
      hash[i][j] = ((i64) rad * hash[i + 1][j] + s[i] - 'a' + 1) % mod[j];
    }
  }
  i32 hash_t[F] = {0, 0, 0, 0};
  for (i32 i = m - 1; i >= 0; --i) {
    for (i32 j = 0; j < F; ++j) {
      hash_t[j] = ((i64) rad * hash_t[j] + t[i] - 'a' + 1) % mod[j];
    }
  }
  i32 *dp = ALLOC (n, i32);
  for (i32 i = 0; i < n; ++i) {
    dp[i] = -1;
  }
  i32 ans = 0;
  for (i32 i = 0; i < d; ++i) {
    if (dp[i] >= 0) continue;
    dp[i] = -2;
    i32 now = i;
    i32 k = 0;
    i32 hash_s[F];
    query (hash, i, i + m, hash_s);
    while (equal (hash_s, hash_t)) {
      ++k;
      now = (now + m) % d;
      if (dp[now] >= 0) {
        k += dp[now];
        break;
      }
      if (dp[now] == -2) {
        puts ("-1");
        return;
      }
      dp[now] = -2;
      query (hash, now, now + m, hash_s);
    }
    ans = MAX (ans, k);
    dp[i] = k;
    query (hash, i, i + m, hash_s);
    now = i;
    while (equal (hash_s, hash_t)) {
      --k;
      now = (now + m) % d;
      if (dp[now] >= 0) break;
      dp[now] = k;
      query (hash, now, now + m, hash_s);
    }
    if (dp[now] < 0) dp[now] = 0;
  }
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}
