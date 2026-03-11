#include <stdio.h>

int a[100000];

int main(void)
{
	int n, i, j;
	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d", a + i);
	}
	for (i = 0, j = 0; i < n; j++) {
		if (j != i) a[j] = a[i];
		while (i < n && a[i] == a[j]) ++i;
	}
	n = j;
	for (i = 0; i < n; i++) {
		printf("%d%c", a[i], " \n"[i + 1 == n]);
	}
	return 0;
}

