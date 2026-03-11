// AOJ 1161: Verbal Arithmetic
// 2018.1.9 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int m, sz;
char str[13][11]; int len[13], max;
char tr['Z'+1], assign['Z'+1];
char idx[10], not0[10], used[10];
int ans;

void calc()
{
	int i, a, s;
	char *p;

	// 下一桁の合計で枝刈りしてみる
	s = 0; for (i = 0; i < m; i++) s += assign[str[i][len[i]]];
	a = assign[str[m][len[m]]];
	if (s % 10 != a) return;

	s = 0;
	for (i = 0; i < m; i++) {
		a = 0, p = str[i];
		while (*p >= 0) a = (a<<3) + (a<<1) + assign[*p++];
		s += a;
	}

	a = 0, p = str[m];
	while (*p >= 0) a = (a<<3) + (a<<1) + assign[*p++];
	if (a == s) ans++;
}

void rec(int k)
{
	int i;

	if (k == sz) { calc(); return; }
	if (!used[0]) {
		if (!not0[k]) used[0] = 1, assign[idx[k]] = 0, rec(k+1), used[0] = 0;
	}
	for (i = 1; i < 10; i++) {
		if (used[i]) continue;
		used[i] = 1, assign[idx[k]] = i, rec(k+1), used[i] = 0;
	}
}

int main()
{
	int n, i;
	char buf[10], *p;

	while (fgets(buf, 10, stdin) && *buf != '0') {
		sz = 0, memset(tr, -1, sizeof(tr));
		memset(not0, 0, sizeof(not0));
		n = atoi(buf), m = n - 1, max = 0;
		for (i = 0; i < n; i++) {
			fgets(str[i], 15, stdin);
			for (p = str[i]; *p > ' '; p++) {
				if (tr[*p] < 0) tr[*p] = sz, idx[sz++] = *p;
			}
			*p = -1; len[i] = p-str[i]-1;
			if (len[i] > 0) not0[tr[*str[i]]] = 1;
			if (i < m && len[i] > max) max = len[i];
		}
		ans = 0;
		if (len[m] == max || len[m] == max+1) {
			memset(used, 0, sizeof(used));
			rec(0);
		}
		printf("%d\n", ans);
	}
	return 0;
}
