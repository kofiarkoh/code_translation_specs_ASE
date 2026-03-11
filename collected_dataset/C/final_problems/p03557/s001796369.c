#include <stdlib.h>
#include <stdio.h>
#include <math.h>

#define rep(n, i) for(int i = 0; i < n; i++)
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

int compare_int(const void *a, const void *b)
{
    return *(int*)a - *(int*)b;
}

int binary_lower(long *a,int n,int k){
    if (a[0] > k) return -1;
    int left = -1;
    int right = n;
	while(right - left>1){
		int mid = (left + right) / 2;
		if(a[mid] < k) left = mid;
		else right = mid;
	}
	return left;
}

void solve(void){
    int n;
    scanf("%d\n", &n);
    long as[n], bs[n], cs[n];
    rep(n, i) scanf("%ld ", &as[i]);
    rep(n, i) scanf("%ld ", &bs[i]);
    rep(n, i) scanf("%ld ", &cs[i]);

    qsort(as, n, sizeof(long), compare_int);
    qsort(bs, n, sizeof(long), compare_int);
    qsort(cs, n, sizeof(long), compare_int);

    int dp[n][3];
    rep(n, i) dp[i][0] = i+1;

    int key_0 = binary_lower(as, n, bs[0]);
    if (key_0 >= 0 && key_0 < n) dp[0][1] = dp[key_0][0];
    else dp[0][1] = 0;

    for (int i = 1; i < n; i++){
        int key = binary_lower(as, n, bs[i]);
        dp[i][1] = dp[i-1][1];
        if (key >= 0 && key < n) dp[i][1] += dp[key][0];
    }

    int key_1 = binary_lower(bs, n, cs[0]);
    if (key_1 >= 0 && key_1 < n) dp[0][2] = dp[key_1][1];
    else dp[0][2] = 0;

    for (int i = 1; i < n; i++){
        int key = binary_lower(bs, n, cs[i]);
        dp[i][2] = dp[i-1][2];
        if (key >= 0 && key < n) dp[i][2] += dp[key][1];
    }

  printf("%d\n", dp[n-1][2]);
  /*
  rep(n, i) printf("%d ", dp[i][0]); printf("\n");
  rep(n, i) printf("%d ", dp[i][1]); printf("\n");
  rep(n, i) printf("%d ", dp[i][2]); printf("\n");
  */

}

int main(void){
  solve();
  return 0;
}
