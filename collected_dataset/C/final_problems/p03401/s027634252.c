#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
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
#define MAX 100002
#define MOD 1000000007

typedef long long int lli;

int main(void) {
  int n;
  scanf("%d", &n);
  int x[MAX] = {0};
  rep(i, 1, n+1) scanf("%d", &x[i]);

  int sum = 0;
  rep(i, 0, n+1) { sum += abs(x[i+1] - x[i]); }

  int sub;
  rep(i, 1, n+1) {
    sub = abs(x[i+1]- x[i]) + abs(x[i] - x[i-1]) - abs(x[i+1] - x[i-1]);
    printf("%d\n", sum - sub);
  }
  
  return 0;
}
