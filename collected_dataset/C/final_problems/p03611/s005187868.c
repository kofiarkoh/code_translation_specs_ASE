#include <stdio.h>
#include <stdlib.h>
int maxi(int a, int b) {
  if(a > b) return a;
  return b;
}
int main() {
  int n;
  int a[100010] = {};
  int ans = 0;
  scanf("%d",&n);
  if (n == 1) {
    printf("1\n");
    return 0;
  }
  else if (n == 2) {
    int a, b;
    scanf("%d %d",&a,&b);
    printf("%d\n",(abs(a-b)>2)?1:2);
    return 0;
  }
  for (int i = 0; i < n; i++) {
    int tmp;
    scanf("%d",&tmp);
    a[tmp]++;
  }
  for (int i = 1; i < 100000; i++) {
    ans = maxi(ans, a[i-1]+a[i]+a[i+1]);
  }
  printf("%d\n",ans);
  return 0;
}