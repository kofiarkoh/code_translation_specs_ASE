#include <stdio.h>

#define SIZE 250
#define NUM 801

int main()
{
	int i, N, A[200001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", &(A[i]));
	for (i = 1; i < N; i++) if (A[i] >= A[i+1]) break;
	if (i == N) {
		printf("1\n");
		fflush(stdout);
		return 0;
	}
	
	int j, k, l = 2, r = N, m, n, count[200001], block[NUM], tmp;
	while (l < r) {
		m = (l + r) / 2;
		n = N / (m - 1);
		for (i = 0; i < n; i++) count[i] = 0;
		for (i = 0; i < (n - 1) / SIZE; i++) block[i] = 0;
		if (A[1] < n) {
			tmp = (A[1] - 1) / SIZE;
			for (i = 0; i < tmp; i++) block[i] = 1;
			for (j = tmp * SIZE; j < A[1]; j++) count[j] = 1;
			k = A[i];
		} else {
			tmp = (n - 1) / SIZE;
			for (i = 0; i < tmp; i++) block[i] = 1;
			for (j = tmp * SIZE; j < n; j++) count[j] = 1;
			k = n;
		}
		for (i = 2; i <= N; i++) {
			if (A[i] >= n) continue;
			else if (A[i] >= k) {
				for (j = k, tmp = k / SIZE + 1; j < tmp * SIZE && j < A[i]; j++) count[j] = 1;
				if (j < A[i]) {
					for (i = tmp, tmp = (A[i] - 1) / SIZE; i < tmp; i++) block[i] = 1;
					for (j = tmp * SIZE; j < A[i]; j++) count[j] = 1;
				}
				k = A[i];
			} else {
				tmp = (A[i] - 1) / SIZE;
				if (block[tmp] == 1) {
					block[tmp] = 0;
					for (j = tmp * SIZE; j < (tmp + 1) * SIZE; j++) count[j] = 1;
				}
				for (j = A[i] - 1; j >= tmp * SIZE && count[j] == m; j--);
				if (j < 0) break;
				else if (j < tmp * SIZE) {
					while (--tmp >= 0 && block[tmp] == 0) {
						for (; j >= tmp * SIZE && count[j] == m; j--);
						if (j >= tmp * SIZE) break;
					}
					if (tmp < 0) break;
					else if (block[tmp] == 1) {
						block[tmp] = 0;
						for (j = tmp * SIZE; j < (tmp + 1) * SIZE; j++) count[j] = 1;
						j--;
					}
				}
				for (count[j++]++; j < A[i]; count[j++] = 1);
				for (tmp = (A[i] - 1) / SIZE, j = A[i]; j < (tmp + 1) * SIZE; j++) count[j] = 0;
			}
		}
		if (i <= N) l = m + 1;
		else r = m;
	}
	printf("%d\n", l);
	fflush(stdout);
	return 0;
}