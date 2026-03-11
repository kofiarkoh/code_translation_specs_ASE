#include<stdio.h>
int main(){
  int n, h, w, ans = 0, a, b;
  scanf("%d %d %d", &n, &h, &w);
  for(int i = 0; i < n; ++i){
    scanf("%d %d", &a, &b);
    if(a >= h && b >= w) ans++;
  }
  printf("%d\n", ans);
  return 0;
}