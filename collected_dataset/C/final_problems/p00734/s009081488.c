#include <stdio.h>

int main() {

	int n, m, i=1, j, k, l = 0, sumt = 0, sumh = 0;
	int taro[101], hanako[101], co1[101], co2[101], ans[202];

	for (i = 0; i < 202; i++) {
		ans[i] = 100;
	}

	while (i) {

		scanf("%d %d", &n, &m);

		if (n == 0 && m == 0)
			break;

		for (i = 0; i < n; i++) {
			scanf("%d", &taro[i]);
			co1[i] = taro[i];
		}

		for (i = 0; i < m; i++) {
			scanf("%d", &hanako[i]);
			co2[i] = hanako[i];
		}

		for (i = 0; i < n; i++) {

			for (j = 0; j < m; j++) {

				co1[i] = hanako[j];
				co2[j] = taro[i];

				for (k = 0; k < n; k++)
					sumt += co1[k];
				for (k = 0; k < m; k++)
					sumh += co2[k];

				if (sumt == sumh && ((co1[i] + co2[j]) < (ans[l] + ans[l + 1]))) {
					ans[l] = taro[i];
					ans[l + 1] = hanako[j];
				}
				else if (ans[l] == 100 && ans[l + 1] == 100)
					ans[l] = -1;

				sumt = 0;
				sumh = 0;

				for (k = 0; k < n; k++)
					co1[k] = taro[k];
				for (k = 0; k < m; k++)
					co2[k] = hanako[k];
			}
		}
		l = l + 2;
	}

	for (i = 0; i < l; i = i + 2) {
		if (ans[i] == -1)
			printf("%d\n", ans[i]);
		else
			printf("%d %d\n", ans[i], ans[i + 1]);
	}
	
	return 0;
}