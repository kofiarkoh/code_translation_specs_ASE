#include <stdio.h>

int a[1000];

int main(void)
{
	int n, q, i, b, e, t, tmp;
	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d", a + i);
	}
	scanf("%d", &q);
	while (q--) {
		scanf("%d%d%d", &b, &e, &t);
		e -= b;
		for (i = 0; i < e; i++) {
			tmp = a[b + i];
			a[b + i] = a[t + i];
			a[t + i] = tmp;
		}
	}
	for (i = 0; i < n; i++) {
		printf("%d%c", a[i], " \n"[i + 1 == n]);
	}
	return 0;
}

