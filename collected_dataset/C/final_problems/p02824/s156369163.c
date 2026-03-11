#include <stdio.h>
#include <stdlib.h>

int cmp(const void* x, const void* y) {
	int a = *(const int*)x, b = *(const int*)y;
	return a > b ? -1 : a < b;
}

int N, M, V, P;
int A[114514];

int main(void) {
	int i;
	int kotae;
	int maki;
	if (scanf("%d%d%d%d", &N, &M, &V, &P) != 4) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &A[i]) != 1) return 1;
	}
	qsort(A, N, sizeof(*A), cmp);
	maki = V - (P - 1);
	if (maki <= 0) maki = 1;
	kotae = P;
	while (kotae < N - maki) {
		if (A[kotae] + M >= A[P - 1]) {
			kotae++;
		} else {
			printf("%d\n", kotae);
			return 0;
		}
	}
	while (kotae < N) {
		if (P - 1 - (kotae - (N - maki)) >= 0 && A[kotae] + M >= A[P - 1 - (kotae - (N - maki))]) {
			kotae++;
		} else {
			break;
		}
		maki++;
	}
	printf("%d\n", kotae);
	return 0;
}

/*
下位の問題に投票しまくって投票されないやつを抜けるか？

入力例3

10 4 8 5
7 2 3 6 1 6 5 4 6 5

↓

7 6 6 6 5 5 4 3 2 1
---------
=======     =======

抜かすことを狙わないやつには投票することで、下位への投票数を減らせる

*/
