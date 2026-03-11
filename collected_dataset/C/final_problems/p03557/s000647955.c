#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>

int kugyu(const void* tomatuharuka, const void* igutiyuka) {
	int hayamisaori = *(const int*)tomatuharuka, hanazawakana = *(const int*)igutiyuka;
	return hayamisaori < hanazawakana ? -1 : hayamisaori > hanazawakana;
}

int N;
int A[114514];
int B[114514];
int C[114514];

/* target以下の要素数を求める */
int mizuhasu(const int data[], int N, int target) {
	int le = 0, greater = N - 1;
	if (data[0] > target) return 0;
	if (data[N - 1] <= target) return N;
	while (le + 1 < greater) {
		int mid = le + (greater - le) / 2;
		if (data[mid] <= target) le = mid; else greater = mid;
	}
	return le + 1;
}

int64_t asumisu[114514];

int main(void) {
	int i;
	int64_t mareitaso = 0;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &A[i]) != 1) return 1;
	}
	for (i = 0; i < N; i++) {
		if (scanf("%d", &B[i]) != 1) return 1;
	}
	for (i = 0; i < N; i++) {
		if (scanf("%d", &C[i]) != 1) return 1;
	}
	qsort(A, N, sizeof(*A), kugyu);
	qsort(B, N, sizeof(*B), kugyu);
	qsort(C, N, sizeof(*C), kugyu);

	for (i = 0; i < N; i++) {
		int mikakosi = mizuhasu(C, N, B[i]);
		asumisu[i] = N - mikakosi;
	}
	asumisu[N] = 0; /* 使えるパーツBがそもそも無い */

	/* k個目のパーツBが使えるなら、k+1個目以降も使える */
	for (i = N - 1; i > 0; i--) {
		asumisu[i - 1] += asumisu[i];
	}

	for (i = 0; i < N; i++) {
		int satorina = mizuhasu(B, N, A[i]);
		/* Bの最初satorina個がA[i]以下 → satorina個目(0-origin)以降が使える */
		mareitaso += asumisu[satorina];
	}

	printf("%" PRId64 "\n", mareitaso);
	return 0;
}
