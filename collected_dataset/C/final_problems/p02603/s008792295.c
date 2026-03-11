#include	<stdio.h>

int		main(void)
{
	int n, a[80];
	int i;
	int j = 0, s = 1000;

	for (i = 0; i < 80; i++) {
		a[i] = 0;
	}

	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d", &a[i]);
	}

	for (i = 0; i < n; i++) {
		if (a[i] < a[i + 1]) {
			j = s / a[i];
			s = s - (j * a[i]);
		}
		else if (a[i] >= a[i + 1] || a[i + 1] == 0) {
			if (j > 0) {
				s = s + (a[i] * j);
				j = 0;
			}
		}
	}

	printf("%d\n", s);

	return 0;
}