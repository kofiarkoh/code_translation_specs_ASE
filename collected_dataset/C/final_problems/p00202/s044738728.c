#include <stdio.h>

#define S 1000001
#define N 30

int dp[S] = {0}; // dp[i] := 商品の金額和がSとなるか 

int main(void) {
	int isprime[S];
	int i,j;
	for(i = 0; i < S; i++) isprime[i] = 1;
	
	isprime[2] = 1;
	for(i = 2; i < S; i++) {
		if (isprime[i]) {
			for(j = i + i; j < S; j += i) {
				isprime[j] = 0;
			}
		}
	}
	while(1) {
		int n, x;
		int prices[N];
		scanf("%d %d", &n, &x);	
		if (n == 0) break;
		for(i = 0; i < n; i++) {
			scanf("%d", &prices[i]);
		}
	
		for(i = 0; i < S; i++) {
			dp[i] = 0;
		}
		
		
		dp[0] = 1;
		for(i = 0; i < S; i++) {
			if (dp[i]) {
				for(j = 0; j < N; j++) {
					if (i + prices[j] <= x) {
						dp[i + prices[j]] = 1;	
					}
				}
			}
		}
		
		int ans = 0;
		for(i = x; i >= 0; i--) {
			if (dp[i] && isprime[i]) {
				ans = i;
				break;
			}
		}
		if (ans) {
			printf("%d\n", ans);
		} else {
			printf("NA\n");
		}
		
	}
	
	return 0;
}