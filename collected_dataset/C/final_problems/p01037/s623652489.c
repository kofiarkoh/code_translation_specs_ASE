// AOJ 1551: A White Wall
// 2019.2.21 bal4u

#include <stdio.h>
#include <stdlib.h>

int wall[102];

int cmp(const void *a, const void *b)
{
	return *(int *)b - *(int *)a;
}

int main()
{
	int i, j, N, M, a, L;

	scanf("%d%d", &N, &M);
	for (i = 0; i < M; i++) {
		scanf("%d%d", &a, &L);
		for (j = 0; j < L; j++) wall[(a + j) % N] = 1;
	}
	for (j = 0; j < N; j++) if (wall[j] == 0) break;
	if (j == N) printf("%d 1\n", N);
	else {
		for (i = 0; i < N; i++) {
			if (wall[(j + i) % N] && wall[(j + i + 1) % N]) {
				wall[(j + i + 1) % N] += wall[(j + i) % N];
				wall[(j + i) % N] = 0;
			}
		}
		qsort(wall, N, sizeof(int), cmp);
		a = 1;
		for (i = 1; i < N; i++) {
			if (wall[i] == wall[i - 1]) a++;
			else {
				printf("%d %d\n", wall[i - 1], a);
				if (wall[i] == 0) break;
				a = 1;
			}
		}
	}
	return 0;
}
