#include <stdio.h>

int a[100000];

int binary_search(int *lb, int *ub, int k) {
	int *m;
	while (ub - lb > 1) {
		m = lb + ((int)(ub - lb) >> 1);
		if (*m <= k) {
			lb = m;
		}
		else {
			ub = m;
		}
	}
	return *lb == k;
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
		puts(binary_search(a, a + n, k) ? "1" : "0");
	}
	return 0;
}

