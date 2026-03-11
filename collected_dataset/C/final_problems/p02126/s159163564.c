#include <stdio.h>
#include <stdlib.h>

int cmp(const void* x, const void* y) {
	int a = *(const int*)x, b = *(const int*)y;
	return a > b ? -1 : a < b;
}

int N, M, C;
int l[114514];

int ballNums[114514];
int* balls[114514];

int asumisu;
int mizuhasu[114514];

int main(void) {
	int i;
	int answer = 0;
	if (scanf("%d%d%d", &N, &M, &C) != 3) return 1;
	for (i = 1; i <= C; i++) {
		if (scanf("%d" ,&l[i]) != 1) return 1;
	}
	for (i = 0; i < N; i++) {
		int c, w;
		int* newBalls;
		if (scanf("%d%d", &c, &w) != 2) return 1;
		newBalls = realloc(balls[c], sizeof(*balls[c]) * (ballNums[c] + 1));
		if (newBalls == NULL) return 2;
		newBalls[ballNums[c]++] = w;
		balls[c] = newBalls;
	}
	for (i = 1; i <= C; i++) {
		if (ballNums[i] > 0) {
			int j;
			qsort(balls[i], ballNums[i], sizeof(*balls[i]), cmp);
			for (j = 0; j < ballNums[i] && j < l[i]; j++) {
				mizuhasu[asumisu++] = balls[i][j];
			}
		}
	}
	qsort(mizuhasu, asumisu, sizeof(*mizuhasu), cmp);
	for (i = 0; i < M && i < asumisu; i++) {
		answer += mizuhasu[i];
	}
	printf("%d\n", answer);
	return 0;
}

