#include<stdio.h>

long long score(int i, int n, long long* a, int* flag) {
  int m = n/i;
  long long ans = 0;
  for (int j=1; j <= m; j++) {
    if (flag[i*j] == 0) {
      ans += a[i*j];
    }
  }
  return ans;
}

int main() {
  int n;
  int i;
  scanf("%d", &n);
  long long a[110] = {};
  int flag[110] = {};
  for (i = 1; i < n+1; i++) {
    scanf("%lld", &a[i]);
  }
  for (i = n; i >= 1; i--) {
    /*printf("%d\n", score(i, n, a, flag));*/
    if (score(i, n, a, flag) < 0) {
      int m = n/i;
      for (int j=1; j<=m; j++) {
	flag[i*j] = 1;
      }
    }
  }
  
  long long ans = 0;
  for (i = 1; i < n+1; i++) {
    if (flag[i] == 0) {
      ans += a[i];
    }
  }
  printf("%lld\n", score(1, n, a, flag));
  return 0;
}
