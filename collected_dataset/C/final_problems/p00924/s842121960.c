#include <stdio.h>
#define min(A,B) ((A) < (B) ? (A) : (B))
#define IMPOSSIBLE 1000

void swap(int* a, int* b) { int c = *a; *a = *b; *b = c; }

/* 配列aを配列bに並び替えるための最小入れ換え回数を返す */
int solve(const int* a, const int* b, int n) {
	int i, j;

	/* 含まれる0と1の個数が等しいか確かめる */
	int a_sum = 0, b_sum = 0;
	for (i = 0; i < n; i++) a_sum += a[i];
	for (i = 0; i < n; i++) b_sum += b[i];
	if (a_sum != b_sum) return IMPOSSIBLE; /* 並び替えても配列は一致しない */

	/* 並び替え */
	int c[15];
	for (i = 0; i < n; i++) c[i] = a[i];

	int cost = 0;
	for (i = 0; i < n; i++) {
		for (j = i; b[i] != c[j] && j < n; j++); /* 目的の要素を探す */
		if (i != j) swap(c + i, c + j);
		cost += j - i;
	}
	return cost;
}

/* 略記された配列を0と1に展開する */
void decompress(int* d, const int* s, int m, int base) {
	int i, j, k;
	for (i = 0, j = 0, k = 0; i < m; i++) {
		k += s[i];
		for (; j < k; j++) d[j] = (i + base) % 2;
	}
}

int main(void) {
	int n, m, b[15], p[15]; /* 入力 */
	int p0[15], p1[15], cost0, cost1;
	int i;

	/* データセット毎の処理 */
	while (scanf("%d%d", &n, &m) == 2 && n >= 1 && n <= 15 && m >= 1 && m <= n) {
		for (i = 0; i < n; i++) scanf("%d", b + i);
		for (i = 0; i < m; i++) scanf("%d", p + i);

		/* p[]を0と1の配列に展開する */
		decompress(p0, p, m, 0);
		decompress(p1, p, m, 1);

		/* 最小入れ換え回数を求める */
		cost0 = solve(p0, b, n);
		cost1 = solve(p1, b, n);

		/* 出力 */
		printf("%d\n", min(cost0, cost1));
	}

	return 0;
}