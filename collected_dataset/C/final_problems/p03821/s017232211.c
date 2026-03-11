#include<stdio.h>
int main() {
  int n, a[100000], b[100000], i, count = 0;
  scanf("%d", &n);
  for(i = 0; i < n; ++i) scanf("%d %d", &a[i], &b[i]);
  for(i = n - 1; i >= 0; --i){
    while((a[i] + count) % b[i] != 0) ++count;
  }
  printf("%d\n", count);
  return 0;
}