#include <stdio.h>
int main() {
	int i, j, k, n, a[100000], b[100000];
	scanf("%d", &n);
	for (i = 0; i < n; i++)scanf("%d", &a[i]);
	for (i = 0;; i++) {
		b[0] = i;
		for (j = 0; j < n - 1; j++) b[j + 1] = 2 * a[j] - b[j];
		if (b[n - 1] + b[0] == 2 * a[n - 1]) {
			for (k = 0; k < n; k++)printf("%d ", b[k]);
			return 0;
		}
	}
}