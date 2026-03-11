#include <stdio.h>

int main(void) {
	int n;
	int a[1024];
	int q;
	int i;
	if (scanf("%d", &n) != 1) return 1;
	for (i = 0; i < n; i++) {
		if (scanf("%d", &a[i]) != 1) return 1;
	}
	if (scanf("%d", &q) != 1) return 1;
	for (i = 0; i < q; i++) {
		int b, e, k;
		int count = 0;
		int j;
		if (scanf("%d%d%d", &b, &e, &k) != 3) return 1;
		for (j = b; j < e; j++) {
			if (a[j] == k) count++;
		}
		printf("%d\n", count);
	}
	return 0;
}

