#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
#include <ctype.h>
typedef long long int ll;

#define rep(i, n) for (int i = 0; i < (int)(n); ++i)
#define INF 1001001001

int comp(const void* a, const void* b){
  return *(int*)a - *(int*)b;
}
ll gcd(ll x, ll y) {
  return y? gcd(y, x%y): x; 
}
int min(int a, int b) { return (a<b)? a: b; }
int max(int a, int b) { return (a>b)? a: b; }

int n, m, a, b, c, t, cst, ans;
int main(void){
  int key[2001], cost[2001], dp[5010];
  rep(i, 2001) {
    key[i] = 0;
    cost[i] = INF;
  }
  rep (i, 5010) {
    dp[i] = INF;
  }
  scanf("%d%d", &n, &m);
  rep(i, m) {
    scanf("%d%d", &a, &b);
    cost[i] = a;
    rep(j, b) {
      scanf("%d", &c);
      key[i] |= 1<<(--c);
    }
    // printf("DEBUG:%d, %x\n", cost[i], key[i]);
  }
  dp[0] = 0;
  rep(s, 1<<n) {
    t = 0;
    cst = 0;
    rep(i, m) {
      // printf("DEBUG:1<<n:%x, 1<<s:%x, i:%x, m:%d\n",1<<n, s, i, m);
      t = s | key[i];
      cst = dp[s] + cost[i];
      dp[t] = min(dp[t], cst);
      // printf("DEBUG:dp[t]:%d\n", dp[t]);
    }
  }
  ans = dp[(1<<n)-1]>=INF? -1: dp[(1<<n)-1];
  printf("%d\n", ans);
  return 0;
}
