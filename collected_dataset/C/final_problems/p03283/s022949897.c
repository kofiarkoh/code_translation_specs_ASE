#include <time.h>
#include <stdio.h>
#include <stdlib.h>

#define M 200000
#define Q 100000

int ll[M], rr[M];

int compare(const void *a, const void *b) {
	int i = *(int *) a;
	int j = *(int *) b;

	return rr[i] - rr[j];
}

int main() {
	static int ii[M], pp[Q], qq[Q];
	int n, m, q, i, j, k;

	scanf("%d%d%d", &n, &m, &q);
	for (i = 0; i < m; i++) {
		scanf("%d%d", &ll[i], &rr[i]);
		ii[i] = i;
	}
	for (i = 0; i < q; i++)
		scanf("%d%d", &pp[i], &qq[i]);
	srand(time(NULL));
	for (i = m - 1; i >= 0; i--) {
		int tmp;

		j = rand() % m;
		tmp = ii[i];
		ii[i] = ii[j];
		ii[j] = tmp;
	}
	qsort(ii, m, sizeof *ii, compare);
	for (i = 0; i < q; i++) {
		k = 0;
		for (j = 0; j < m && rr[ii[j]] <= qq[i]; j++)
			if (ll[ii[j]] >= pp[i])
				k++;
		printf("%d\n", k);
	}
	return 0;
}
