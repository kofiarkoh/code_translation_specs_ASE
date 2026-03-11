#include <stdio.h>
#include <stdlib.h>
#define P 1000000007

long long *dp[2], t = 0;
int forbidden(int c){return c%125==7||c%125==11||c%125==51||(c%25==11&&!(c/125))||(c%5==1&&c/25==2);}
long long *alloc() {
	int i;
	long long *p = (long long*)malloc(625*sizeof(long long));
	for(i = 0; i < 625; i++)
		p[i] = 0;
	return p;
}
int main() {
	int n, nx, i, j;
	scanf("%d", &n);
	for(i = 0; i < 2; i++)
		dp[i] = alloc();
	dp[0][624] = 1;
	while(n--) {
		for(i = 0; i < 625; i++)
			for(j = 0; j < 4; j++) {
				nx = (i % 125) * 5 + j;
				if(!forbidden(nx))
					dp[1][nx] = (dp[1][nx] + dp[0][i]) % P;
			}
		free(dp[0]);
		dp[0] = dp[1];
		dp[1] = alloc();
	}
	for(i = 0; i < 625; i++)
		t = (t + dp[0][i]) % P;
	printf("%d", t);
	return 0;
}