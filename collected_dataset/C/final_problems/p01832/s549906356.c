// AOJ 2731 Shifting a Matrix
// 2018.2.11 bal4u

#include <stdio.h>
#include <ctype.h>
#include <string.h>

#define SIZE 700

int N;
typedef struct { short r, c; } T;
T ans[101][101], t0[101][101];
T tbl[SIZE+1][101][101];
char used[SIZE+1]; int top;
char s[1002], *p;

char *seq(T res[101][101], char *s);

// malloc
int new() { int i, k;
	used[k=i=top] = 1;
	while (++i < SIZE) if (!used[i]) { top = i; break; }
	return k;
}

// free
void dis(int k) { used[k] = 0; if (k < top) top = k; }

char *in(int *res, char *p) { int n = 0;
	while (isdigit(*p)) n = (n<<3)+(n<<1) + (*p++ & 0xf); *res = n; return p;
}

void calc(T a[101][101], T b[101][101])
{
	int r, c;
	T t[101][101];

	for (r = 0; r < N; r++) for (c = 0; c < N; c++) {
		t[r][c] = a[b[r][c].r][b[r][c].c];
	}
	memcpy(a, t, sizeof(t));
}

char *ope(T a[101][101], char *s)
{
	int k, r, c;
	char x;

	x = *s++;
	s = in(&k, s), k--;
	memcpy(a, t0, sizeof(t0));

	if      (x == 'L') { for (c = 0; c < N; c++) a[k][c].c = c+1; a[k][N-1].c = 0; }
	else if (x == 'R') { for (c = 0; c < N; c++) a[k][c].c = c-1; a[k][0].c = N-1; }
	else if (x == 'U') { for (r = 0; r < N; r++) a[r][k].r = r+1; a[N-1][k].r = 0; }
	else               { for (r = 0; r < N; r++) a[r][k].r = r-1; a[0][k].r = N-1; }
	return s;
}

char *rep(T a[101][101], char *s)
{
	int n, _b;
	T (*b)[101];
	
	b = tbl[_b=new()];
	s = seq(b, s)+1;
	s = in(&n, s);
	memcpy(a, t0, sizeof(t0));

	while (n) {
		if (n & 1) calc(a, b);
		calc(b, b);
		n >>= 1;
	}
	dis(_b);
	return s;
}

char *seq(T res[101][101], char *s)
{
	int _t;
	T (*t)[101];

	memcpy(res, t0, sizeof(t0));
	t = tbl[_t=new()];
	while (*s == '(' || isalpha(*s)) {
		if (*s == '(') s = rep(t, s + 1);
		else           s = ope(t, s);
		calc(res, t);
	}
	dis(_t);
	return s;
}

int main()
{
	int L, r, c;
	
	fgets(p=s, 16, stdin), p = in(&N, p), in(&L, p+1);
	fgets(s, 1002, stdin);

	for (r = 0; r < N; r++) for (c = 0; c < N; c++)
		t0[r][c].r = r, t0[r][c].c = c;

	seq(ans, s);
	for (r = 0; r < N; r++) {
		printf("%d", N*ans[r][0].r + ans[r][0].c + 1);
		for (c = 1; c < N; c++)
			printf(" %d", N*ans[r][c].r + ans[r][c].c + 1);
		putchar('\n');
	}
	return 0;
}
