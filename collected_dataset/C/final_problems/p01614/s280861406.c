#include<stdio.h>
#include <string.h>
#define NMAX 393
#define max(a,b) ((a) > (b) ? (a) : (b))

int main(void ){
  int i, j, n, m, s, l, p, w, dp[NMAX+1], phrase[NMAX], ans[NMAX];
  scanf("%d", &n);
  memset(phrase, 0, sizeof(phrase));
  memset(dp, 0, sizeof(dp));
  for(i = 0; i < n; i++){
    scanf("%d %d %d", &s, &l, &p);
    for(j = s; j <= l; j++) phrase[j-1] = max(phrase[j-1], p);
  }
  for(i = 1; i <= NMAX; i++) {
    if(phrase[i-1]){
      for(j = i; j <= NMAX; j++) {
        dp[j] = max(dp[j], dp[j-i] + phrase[i-1]);
      }
    }
  }
  scanf("%d", &m);
  for(i = 0; i < m; i++){
    scanf("%d", &w);
    if(!dp[w]){
      printf("-1\n");
      return 0;
    }
    ans[i] = dp[w];
  }
  for(i = 0; i < m; i++) printf("%d\n", ans[i]);
}
