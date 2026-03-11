// AOJ 2731 Shifting a Matrix
// 2018.2.11 bal4u

#include <stdio.h>
#include <ctype.h>
#include <string.h>

#define SIZE 700

int N, N2;
short ans[10001], t0[10001], tmp[10001];
short mat[SIZE+1][10001];
char used[SIZE+1]; int top;
char s[1002], *p;

char *seq(short *res, char *s);

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

void calc(short *a, short *b)
{
	int i;
	for (i = 0; i < N2; i++) tmp[i] = a[b[i]];
	memcpy(a, tmp, sizeof(tmp));
}

char *ope(short *a, char *s)
{
	int k, r, c;
	char x;

	x = *s++;
	s = in(&k, s), k--;

	memcpy(a, t0, sizeof(t0));
	if      (x == 'L') { for (c = 0; c < N; c++) a[k*N+c] = k*N+c+1; a[k*N+N-1] = k*N; }
	else if (x == 'R') { for (c = 0; c < N; c++) a[k*N+c] = k*N+c-1; a[k*N] = k*N+N-1; }
	else if (x == 'U') { for (r = 0; r < N; r++) a[r*N+k] = (r+1)*N+k; a[(N-1)*N+k] = k; }
	else               { for (r = 0; r < N; r++) a[r*N+k] = (r-1)*N+k; a[k] = (N-1)*N+k; }
	return s;
}

char *rep(short *a, char *s)
{
	int n, _b;
	short *b;

	b = mat[_b=new()];
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

char *seq(short *res, char *s)
{
	int _t;
	short *t;

	memcpy(res, t0, sizeof(t0));
	t = mat[_t=new()];
	memcpy(t, t0, sizeof(t0));
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
	int L, i, r, c, _r;
	
	fgets(p=s, 16, stdin), p = in(&N, p), in(&L, p+1), N2 = N*N;
	fgets(s, 1002, stdin);
	for (i = 0; i < N2; i++) t0[i] = i;

	seq(ans, s);
	for (_r = 0, r = 0; r < N; r++, _r += N) {
		printf("%d", 1+ans[_r]);
		for (c = 1; c < N; c++)	printf(" %d", 1+ans[_r+c]);
		putchar('\n');
	}
	return 0;
}
