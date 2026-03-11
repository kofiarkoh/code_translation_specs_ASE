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
#define INF 1000000009
typedef long long int lli;

int main(void) {
  int n, x, y;
  scanf("%d %d %d", &n, &x, &y);

  int count[MAX] = {0};
  rep(i, 1, n+1) {
    rep(j ,i, n+1) {
      int l = min(min(j-i, abs(x-i)+abs(y-j)+1), abs(y-i)+abs(x-j)+1);
      count[l]++;
    }
  }
  rep(i, 1, n) {
    printf("%d\n", count[i]);
  }
  return 0;
}
