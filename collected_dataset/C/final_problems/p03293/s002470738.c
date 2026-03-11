#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */
int cmp_char(const void *a, const void *b){return *(char*)a - *(char*)b;} /* a,b,c,d.. */
int cmp_str(const void *a, const void *b){return strcmp(*(const char **)a, *(const char **)b);} /* aaa,aab.. */
#define min(a,b) (a < b ? a: b)
#define max(a,b) (a > b ? a: b)
#define rep(i, l, r) for (int i = l; i < r; i++)
#define MAX 100001
#define MOD 1000000007

typedef long long int lli;

int main(void) {
  char s[101], t[101];
  scanf("%s", s);
  scanf("%s", t);
  int n = strlen(s);
  int ok = 0;
  rep(first, 0, n) {
    if ( s[first] != t[0] ) continue;
    int ti = 1;
    int si = first + 1;
    int judge = 1;
    while (si % n != first) {
      if ( s[si % n] != t[ti % n] ) {
        judge = 0;
        break;
      }
      si++; ti++;
    }
    if ( judge ) {
      ok = 1;
      break;
    }
  }
  printf( ok ? "Yes\n" : "No\n" );
  return 0;
}
