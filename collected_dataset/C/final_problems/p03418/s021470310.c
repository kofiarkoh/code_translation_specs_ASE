#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
int acs(const void *a, const void *b) { 
  return *(int*)a - *(int*)b;
} /* 1,2,3,4.. */
int des(const void *a, const void *b) {
  return *(int*)b - *(int*)a;
} /* 8,7,6,5.. */
int cmp_char(const void *a, const void *b) { 
  return *(char*)a - *(char*)b;
} /* a,b,c,d.. */
int cmp_str(const void *a, const void *b) {
  return strcmp(*(const char **)a, *(const char **)b);
} /* aaa,aab.. */
#define min(a,b) (a < b ? a : b)
#define max(a,b) (a > b ? a : b)
#define rep(i, l, r) for (int i = l; i < r; i++)
#define MAX 100001
#define MOD 1000000007

typedef long long int lli;

int main(void) {
  lli N, K;
  scanf("%lld %lld", &N, &K);

  lli ans = 0;
  for ( lli b = 1; b <= N; b++ ) {
    lli r = N % b;
    lli p = N / b;
    ans += max(r - K + 1, 0);
    ans += max(b - K, 0) * p;
    if ( K == 0 ) ans--;
  }
  printf("%lld\n", ans);
  return 0;
}
