// AOJ 1307 Towns along a Highway
// 2019.2.28 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif

int in()   // 正整数の入力
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)
{
	int i;
	char ob[20];

	i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
	while (i--) pc(ob[i]);
}

void outs(char *s) { while (*s) pc(*s++); }
#define ABS(x) ((x)>=0?(x):-(x))

int N;
int d[200], dsz, max;
char f[402];     // 入力した値の個数を記録する
int ans[20][20], asz;
int try[20];     // ansの候補

int check(int id, int val)
{
	int i, k;
	char ff[402];

	memcpy(ff, f, 1+max);
	for (i = 0; i < id; i++) {
		k = ABS(try[i]-val);
		if (ff[k] == 0) return 0;
		ff[k]--;
	}
	return 1;
}

int cmp(const void *a, const void *b) {	return *(int *)a - *(int *)b; }

void combi(int id)
{
	int i, k;
	char ff[402];

	if (id == N) {
		memcpy(ans[asz], try, N * sizeof(int));
		qsort(ans[asz], N, sizeof(int), cmp);
		for (i = 0; ; i++) {
			if (i == asz) { asz++; break; }
			if (!memcmp(ans[i], ans[asz], N * sizeof(int))) break;
		}
		return;
	}

	for (k = max; !f[k]; k--);
	memcpy(ff, f, 1+max);
	if (check(id, max-k)) {
		try[id] = max - k;
		for (i = 0; i < id; i++) f[ABS(try[i] - try[id])]--;
		combi(id + 1);
		memcpy(f, ff, 1+max);
	}

	if (check(id, k)) {
		try[id] = k;
		for (i = 0; i < id; i++) f[ABS(try[i] - try[id])]--;
		combi(id + 1);
		memcpy(f, ff, 1+max);
	}
}

int main()
{
	int i, j;

	while (N = in()) {
		dsz = N * (N - 1) >> 1;
		memset(f, 0, sizeof(f));
		for (i = 0; i < dsz; i++) {
			d[i] = in(), f[d[i]]++;
		}
		if (dsz == 1) out(d[0]), pc('\n');
		else {
			max = d[0];
			try[0] = 0, try[1] = max, f[max]--;  // 最左端と最右端の距離が最大
			asz = 0;
			combi(2);

			// 結果表示
			for (i = 0; i < asz; i++) {
				out(ans[i][1]);
				for (j = 2; j < N; j++) pc(' '), out(ans[i][j] - ans[i][j - 1]);
				pc('\n');
			}
		}
		outs("-----\n");
	}
	return 0;
}

