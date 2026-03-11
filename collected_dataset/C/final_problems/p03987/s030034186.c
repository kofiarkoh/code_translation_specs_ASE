#include <stdio.h>

int main(void) {
	int arr[200000];
	int n;
	long long int total = 0;
	int i, l, r, m;

	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d", &arr[i]);
	}

	for (l = 0; l < n; l++) {
		m = arr[l];
		total += m;
		for (r = l + 1; r < n; r++) {
			if (m > arr[r]) {
				m = arr[r];
			}
			total += m;
		}
	}

	printf("%lld\n", total);
}
