#include <stdio.h>
#define MOD 1000000007

int n, m;
int x[100000], y[100000];
long long int scale_x[100000], scale_y[100000];

int main(void) {
	int i, j;
	long long int ans = 0;
	long long int area;

	scanf("%d%d", &n, &m);
	for (i = 0; i < n; i++) {
		scanf("%d", &x[i]);
		scale_x[i] = i * (n - i);
	}
	for (i = 0; i < m; i++) {
		scanf("%d", &y[i]);
		scale_y[i] = i * (m - i);
	}

	for (i = 1; i < n; i++) {
		for (j = 1; j < m; j++) {
			area = ((long long int)(x[i] - x[i - 1]) * (y[j] - y[j - 1])) % MOD;
			ans = (ans + area * scale_x[i] * scale_y[j]) % MOD;
		}
	}
	printf("%lld\n", ans);

	return 0;
}