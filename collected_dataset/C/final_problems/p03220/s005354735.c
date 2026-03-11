#include <stdio.h>
#include <math.h>

int main(void) {
	int n, t, a, i, ans;
	int h[1000];
	double temp, ans_h;
	scanf("%d", &n);
	scanf("%d%d", &t, &a);
	for (i = 0; i < n; i++) {
		scanf("%d", &h[i]);
		temp = t - h[i] * 0.006;
		if (i == 0) {
			ans = i + 1;
			ans_h = temp;
		}
		else {
			if (fabs(a - temp) < fabs(a - ans_h)) {
				ans_h = temp;
				ans = i + 1;
			}
		}

	}
	printf("%d", ans);
	return 0;
}