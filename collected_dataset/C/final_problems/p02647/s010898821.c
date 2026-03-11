#include <stdio.h>

int main(void) {
	int n, k;
	int a[200000] = { 0 };
	int b[200000] = { 0 };
	int c[200000] = { 0 };

	scanf("%d", &n);
	scanf("%d", &k);

	for (int i = 0; i < n; i++) {
		scanf("%d", &a[i]);
		b[i] = 1;
		c[i] = 1;
	}

	int mode = 0;
	for (int i = 0; i < k; i++) {
		if (mode == 0) {
			mode = 1;
			for (int j = 0; j < n; j++) {
				c[j] = 1;
				for (int x = 1; x <= a[j]; x++) {
					if (j - x >= 0) {
						b[j - x]++;
					}
					if (j + x < n) {
						b[j + x]++;
					}
				}
			}
		}
		else if(mode==1){
			mode = 2;
			for (int j = 0; j < n; j++) {
				a[j] = 1;
				for (int x = 1; x <= b[j]; x++) {
					if (j - x >= 0) {
						c[j - x]++;
					}
					if (j + x < n) {
						c[j + x]++;
					}
				}
			}
		}
		else {
			mode = 0;
			for (int j = 0; j < n; j++) {
				b[j] = 1;
				for (int x = 1; x <= c[j]; x++) {
					if (j - x >= 0) {
						a[j - x]++;
					}
					if (j + x < n) {
						a[j + x]++;
					}
				}
			}
		}
	}

	for (int i = 0; i < n; i++) {
		if (mode == 0) {
			printf("%d ", a[i]);
		}
		else if (mode == 1) {
			printf("%d ", b[i]);
		}
		else {
			printf("%d ", c[i]);
		}
	}

	return 0;
}