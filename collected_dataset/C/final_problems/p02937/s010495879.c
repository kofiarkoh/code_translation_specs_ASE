#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>
#include <limits.h>
#define Max(a, b) ((a) > (b) ? (a) : (b))
#define Min(a, b) ((a) > (b) ? (b) : (a))
#define abs(x) ((x) > 0 ? (x) : -(x))
#define MOD 1000000007 //10^9 + 7
#define endl printf("\n")
typedef long long ll;
#define MAX_N (1 << 17)

int nal[100010][26];

int
main(int argc, const char *argv[])
{
  char s[100001], t[100001];
  scanf("%s", s);
  scanf("%s", t);

  int al[30] = {};
  int n = strlen(s), m = strlen(t);
  for (int i = 0; i < n; i++) al[s[i] - 'a']++;
  for (int i = 0; i < m; i++) {
    if (al[t[i] - 'a'] == 0) {
      printf("-1\n");
      return 0;
    }
  }

  for (int i = 0; i < 26; i++) nal[n][i] = -1;
  nal[n][s[n - 1] - 'a'] = n;
  for (int i = n - 1; i >= 1; i--) {
    for (int j = 0; j < 26; j++) {
      nal[i][j] = nal[i + 1][j];
    }
    nal[i][s[i - 1] - 'a'] = i;
  }

  ll k = 0; int e = 0;
  while (e < m) {
    if (nal[(int)(k % (ll)n) + 1][t[e] - 'a'] != -1) {
      k += nal[(int)(k % (ll)n) + 1][t[e] - 'a'] - k % (ll)n;
      e++;
    } else {
      k += (ll)n - k % (ll)n;
    }
  }

  printf("%lld\n", k);

  return 0;
}
  

  

  
