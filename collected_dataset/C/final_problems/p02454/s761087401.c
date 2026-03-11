#include <stdio.h>

int a[100000];

int lower_bound(int lb, int ub, int k) {
	int m;
	while (ub - lb > 1) {
		m = (lb + ub) >> 1;
		if (a[m] < k) {
			lb = m;
		}
		else {
			ub = m;
		}
	}
	return ub;
}

int upper_bound(int lb, int ub, int k) {
	int m;
	while (ub - lb > 1) {
		m = (lb + ub) >> 1;
		if (a[m] <= k) {
			lb = m;
		}
		else {
			ub = m;
		}
	}
	return ub;
}

int main(void)
{
	int n, q, k, i;
	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d", a + i);
	}
	scanf("%d", &q);
	while (q--) {
		scanf("%d", &k);
		printf("%d %d\n", lower_bound(-1, n, k), upper_bound(-1, n, k));
	}
	return 0;
}

