#include <stdio.h>
int main (void) {
	int n, h, w, r = 0;
	int a[2000], b[2000];
	scanf("%d %d %d", &n, &h, &w);
	for (int i = 0; i < n; i++) {
		scanf("%d %d", &a[i], &b[i]);
	}
	for (int i = 0; i < n; i++) {
		if (a[i] >= h && b[i] >= w) {
			r++;
		}
	}
	printf("%d\n", r);
	return 0;
}
