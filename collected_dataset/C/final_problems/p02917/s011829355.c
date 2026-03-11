#include <stdio.h>

int min(int a, int b){
  if(a < b){
    return a;
  }else{
    return b;
  }
}

int main(){
  int n;
  scanf("%d", &n);
  int a[n], b[n - 1];
  for(int i = 0; i < n - 1; i++){
    scanf("%d", &b[i]);
  }
  a[0] = b[0];
  a[n - 1] = b[n - 2];
  for(int i = 1; i < n - 1; i++){
    a[i] = min(b[i - 1], b[i]);
  }
  int ans = 0;
  for(int i = 0; i < n; i++){
    ans += a[i];
  }
  printf("%d\n", ans);
}
