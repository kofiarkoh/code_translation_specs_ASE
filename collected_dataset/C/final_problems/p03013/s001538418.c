#include <stdio.h>
#define max(x, y) ((x)<(y) ? (y) : (x))

int main(){
  long mod = 1e9+7;
  int n, m;  scanf("%d %d", &n, &m);
  int a[100005] = {0}; a[m+1] = n+1;  a[0] = -10000;
  for(int i=1; i<=m; i++){
    scanf("%d", a+i);
    if(a[i] - a[i-1] <= 1){
      printf("0\n");
      return 0;
    }
  }
  if(a[m+1] - a[m] <= 1){ printf("0\n"); return 0; }
  a[0] = 1;
  int now = 0, b[100005] = {0}, maxi = 0;
  for(int i=1; i<=m+1; i++){
    b[i] = a[i] - 1 - now;
    maxi = max(maxi, b[i]);
    now = a[i] + 1;
  }

  long fibo[100005] = {0};  fibo[1] = fibo[0] = 1;
  for(int i=2; i<=maxi; i++)
    fibo[i] = (fibo[i-1] + fibo[i-2]) % mod;

  long ans = 1;
  for(int i=1; i<=m+1; i++)
    if(b[i])
      ans = (ans * fibo[b[i]]) % mod;


  printf("%ld\n", ans);
  return 0;
}
