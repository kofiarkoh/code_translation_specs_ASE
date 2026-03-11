#include<stdio.h>

int main(void) {
  int n,count = 1;
  int cnt = 0;
  scanf("%d", &n);
  int a[n];
  for(int i = 0; i < n; i++) scanf("%d", &a[i]);
  for(int i = 0; i < n; i++) {
    if(a[i] == count) count++;
    else cnt++;
  }
  if(count == 1) printf("-1\n");
  else printf("%d\n", cnt);
  
  return 0;
}