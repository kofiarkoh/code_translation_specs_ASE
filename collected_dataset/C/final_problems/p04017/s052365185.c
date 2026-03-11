#include <stdio.h>

#define N_MAX 110000

int N;
int x[N_MAX];
int L;
int Q;
int a, b;

int ikeru_left[30][N_MAX];
int ikeru_right[30][N_MAX];

void maesyori(void) {
	int i, j;
	/* 各ホテルから1日で行けるホテルを求める */
	for (i = 0; i < N; i++) {
		int left, right, mid;
		/* 左へ */
		left = 0; ikeru_left[0][i] = right = i;
		while (left <= right) {
			mid = (left + right) / 2;
			if (x[i] - x[mid] <= L) {
				/* 行ける */
				right = mid - 1;
				if (mid < ikeru_left[0][i]) ikeru_left[0][i] = mid;
			} else {
				/* 行けない */
				left = mid + 1;
			}
		}
		/* 右へ */
		ikeru_right[0][i] = left = i; right = N - 1;
		while (left <= right) {
			mid = (left + right) / 2;
			if (x[mid] - x[i] <= L) {
				/* 行ける */
				left = mid + 1;
				if (ikeru_right[0][i] < mid) ikeru_right[0][i] = mid;
			} else {
				/* 行けない */
				right = mid - 1;
			}
		}
	}
	/* 各ホテルから2**k日で行けるホテルを求める */
	for (j = 1; j < 30; j++) {
		for (i = 0; i < N; i++) {
			ikeru_left[j][i] = ikeru_left[j - 1][ikeru_left[j - 1][i]];
			ikeru_right[j][i] = ikeru_right[j - 1][ikeru_right[j - 1][i]];
		}
	}
}

int motomeru(int start, int day, int ikeru[][N_MAX]) {
	int cur = start;
	int i;
	for (i = 0; i < 30; i++) {
		if ((day >> i) & 1) cur = ikeru[i][cur];
	}
	return cur;
}

int main(void) {
	int i, query_count;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &x[i]) != 1) return 1;
	}
	if (scanf("%d%d", &L, &Q) != 2) return 1;
	maesyori();
	for (query_count = 0; query_count < Q; query_count++) {
		int left, right, mid;
		int ret = N + 10;
		if (scanf("%d%d", &a, &b) != 2) return 1;
		a--; b--;
		left = 0; right = N; /* 着けるから絶対付くまで進捗する */
		while (left <= right) {
			int judge;
			mid = (left + right) / 2;
			if (a < b) {
				judge = motomeru(a, mid, ikeru_right) >= b;
			} else {
				judge = motomeru(a, mid, ikeru_left) <= b;
			}
			if (judge) {
				if (mid < ret) ret = mid;
				right = mid - 1;
			} else {
				left = mid + 1;
			}
		}
		printf("%d\n", ret);
	}
	return 0;
}
