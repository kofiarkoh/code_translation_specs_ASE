#include <stdio.h>

int mizuyari(int l, int r, int *h) {
	if (l > r) return 0;
	if (l == r) return h[l];

	int i, min = 100, count;

	for (i = l; i <= r; i++) {
		if (h[i] < min) min = h[i];
	}

	count = min;

	for (i = l; i <= r; i++) {
		h[i] = h[i] - min;
		if (h[i] == 0) {
			count = count + mizuyari(l, i - 1, h);
			l = i + 1;
		}
	}

	count = count + mizuyari(l, r, h);

	return count;
}

int main() {
	int N, h[100], i, ans;

	scanf("%d", &N);
	for (i = 0; i < N; i++) {
		scanf("%d", &h[i]);
	}

	ans = mizuyari(0, N - 1, h);

	printf("%d", ans);

	return 0;
}