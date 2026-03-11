// AOJ 1326: Stylish
// 2018/1/21

#include <stdio.h>
#include <string.h>

#define INF 100

typedef struct { int r, c, s, i; } T;	// i: indent
T ptbl[11], *pend;
T qtbl[11], *qend;

int ind[11];
char p[11][85];
char q[11][85];

int in()
{
	int n = 0;
	int c = getchar();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar();
	while (c >= '0');
	return n;
}

void calc()
{
	int R, C, S, i, k, r, c, s;
	char *pt;
	T *t;

	r = c = s = 0;
	i = 0; for (t = ptbl; t < pend; t++) {
		pt = p[i++], k = 0; while (*pt == '.') pt++, k++;
		t->r = r, t->c = c, t->s = s, t->i = k;

		while (*pt > ' ') {
			if      (*pt == '(') r++;
			else if (*pt == ')') r--;
			else if (*pt == '{') c++;
			else if (*pt == '}') c--;
			else if (*pt == '[') s++;
			else if (*pt == ']') s--;
			pt++;
		}
	}

	r = c = s = 0;
	i = 0; for (t = qtbl; t < qend; t++) {
		t->r = r, t->c = c, t->s = s;
		pt = q[i++]; while (*pt > ' ') {
			if      (*pt == '(') r++;
			else if (*pt == ')') r--;
			else if (*pt == '{') c++;
			else if (*pt == '}') c--;
			else if (*pt == '[') s++;
			else if (*pt == ']') s--;
			pt++;
		}
	}

	for (R = 1; R <= 20; R++) for (C = 1; C <= 20; C++) for (S = 1; S <= 20; S++) {
		for (t = ptbl; t < pend; t++) {
			if (R * t->r + C * t->c + S * t->s != t->i) goto next;
		}

		for (i = 0, t = qtbl; t < qend; t++, i++) {
			if (ind[i] == -1) continue;
			k = R * t->r + C * t->c + S * t->s;
			if      (ind[i] == INF) ind[i] = k;
			else if (ind[i] != k  ) ind[i] = -1;
		}
next:;
	}
}

int main()
{
	int P, Q, i;

	while (P = in()) {
		Q = in();
		pend = ptbl+P, qend = qtbl+Q;
		for (i = 0; i < P; i++) fgets(p[i], 85, stdin);
		for (i = 0; i < Q; i++) fgets(q[i], 85, stdin);

		for (i = 0; i < Q; i++) ind[i] = INF;
		calc();
		printf("0"); for (i = 1; i < Q; i++) printf(" %d", ind[i]);
		putchar('\n');
	}
	return 0;
}
