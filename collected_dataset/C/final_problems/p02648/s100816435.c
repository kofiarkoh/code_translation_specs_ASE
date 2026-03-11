#include <stdio.h>
#define max(x, y) ((x)<(y) ? (y) : (x))

int main(){
  int n;  scanf("%d", &n);
  int v[(1<<18)+22], w[(1<<18)+22];
  for(int i=1; i<=n; i++)
    scanf("%d %d", &v[i], &w[i]);

  int q;  scanf("%d", &q);
  for(int i=0; i<q; i++){
    int u, l;  scanf("%d %d", &u, &l);
    int dp[100005] = {0};
    while(u){
      for(int vi=l; vi>=w[u]; vi--){
	dp[vi] = max(dp[vi], dp[vi-w[u]]+v[u]);
      }
      u = u / 2;
    }
    printf("%d\n", dp[l]);
  }
  
  return 0;
}
