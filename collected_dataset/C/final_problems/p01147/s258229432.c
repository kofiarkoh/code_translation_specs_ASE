// AOJ 2022 Princess, a Cryptanalyst
// 2018.3.16 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int N;
char word[10][12]; int sz[12];

// 単語間の一致関係
// [a][b]=k 単語bの先頭k文字が単語aの末尾と一致
char in[10][10];

char ans[110]; int len;
char cand[110];
char f[10];

int naiveMatch(char *text, int len, char *pattern, int patlen)
{
	char c;
	char *tp, *pp;
	char *textEnd, *patEnd;

	if (len < patlen) return 0;

	textEnd = text + len - patlen + 1;
	patEnd = pattern + patlen;

	c = *pattern++;
	while (text != textEnd) {
		if (*text++ == c) {
			tp = text;
			pp = pattern;
			do if (pp == patEnd) return 1;
			while (*tp++ == *pp++);
		}
	}
	return 0;
}

void calc()
{
	int i, j, k;
	char *p;

	memset(in, 0, sizeof(in));
	for (i = 0; i < N; i++) {
		for (j = 0; j < N; j++) if (i != j) {
			for (k = sz[i] - 1, p = word[i] + 1; k; k--, p++) {
				if (!memcmp(p, word[j], k)) {
					in[i][j] = k;
					break;
				}
			}
		}
	}
}

void rec(int n, int pre, int w)
{
	int i, t;

	if (n == N) {
		if (w < len || (w == len && memcmp(ans, cand, w) > 0)) {
			memcpy(ans, cand, w), len = w, ans[w] = 0;
		}
		return;
	}
	for (i = 0; i < N; i++) if (!f[i]) {
		t = sz[i] - in[pre][i];
		if (t < len) {
			f[i] = 1;
			memcpy(cand + w, word[i] + in[pre][i], t);
			rec(n + 1, i, w + t);
			f[i] = 0;
		}
	}
}

int main()
{
	int i, j;

	while (scanf("%d", &N) && N) {
		for (i = 0; i < N; i++) {
			scanf("%s", word[i]);
			sz[i] = strlen(word[i]);
		}

		// 完全一致する単語の排除
		memset(f, 0, sizeof(f));
		for (i = 0; i < N; i++) {
			if (f[i]) continue;
			for (j = i + 1; j < N; j++) if (!f[j]) {
				if (naiveMatch(word[i], sz[i], word[j], sz[j])) f[j] = 1;
				else if (naiveMatch(word[j], sz[j], word[i], sz[i])) {
					f[i] = 1;
					break;
				}
			}
		}
		i = 0; for (j = 0; j < N; j++) if (!f[j]) {
			if (j != i) strcpy(word[i], word[j]), sz[i] = sz[j];
			i++;
		}
		N = i;

		calc();

		len = 10000;
		memset(f, 0, sizeof(f));
		for (i = 0; i < N; i++) {
			f[i] = 1;
			memcpy(cand, word[i], sz[i]);
			rec(1, i, sz[i]);
			f[i] = 0;
		}
		puts(ans);
	}
	return 0;
}
