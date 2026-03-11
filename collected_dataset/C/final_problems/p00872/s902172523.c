// AOJ 1293: Common Polynomial
// 2017.12.23 bal4u@uu

#include <stdio.h>
//#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX 10

int a[MAX+2];
int b[MAX+2];

char *parse();

void add(int *a, int *b, int op)
{
	int i;
	for (i = 0; i <= MAX; i++) {
		if (op) a[i] += b[i];
		else    a[i] -= b[i];
	}
}

void mul(int *a, int *b)
{
	int i, j;
	int t[MAX+2];

	memset(t, 0, sizeof(t));
	for (i = 0; i <= MAX; i++) for (j = 0; j <= MAX; j++) {
		if (i + j > MAX) break;
		t[i+j] += a[i]*b[j];
	}
	memcpy(a, t, sizeof(t));
}

char *number(int *k, char *p)
{
	int x = 0;
	while (isdigit(*p)) x = 10*x + (*p++ & 0xf);
	*k = x;
	return p;
}

char *primary(int *a, char *p)
{
	int c, id;

	if (*p == '(') {
		p = parse(a, p+1);
		if (*p == ')') p++;
	} else if (*p < ' ') return p;
	else {
		id = 0, c = 1;
		if (isdigit(*p)) p = number(&c, p);
		else if (*p == 'x') {
			p++, id = 1;
			if (*p == '^') p = number(&id, p+1);
		}
		if (id >= 0 && id <= MAX) a[id] = c;
	}
	return p;
}

char *factor(int *a, char *p)
{
	int n, minus = 0;
	int t[MAX+2];

	if (*p == '-') p++, minus = 1;
	p = primary(a, p);
	if (*p == '^') {
		p = number(&n, p+1);
		memcpy(t, a, sizeof(t));
		while (--n) mul(t, a);
		memcpy(a, t, sizeof(t));
	}
	if (minus) for (n = 0; n <= MAX; n++) a[n] = -a[n];
	return p;
}

char *term(int *a, char *p)
{
	int t[MAX+2];

	p = factor(a, p);
	while (1) {
		if (*p == ')' || *p == '+' || *p == '-') break;
		if (*p < ' ') break;

		memset(t, 0, sizeof(t));
		p = factor(t, p);
		mul(a, t);
	}
	return p;
}

char *parse(int *a, char *p)
{
	int op;
	int t[MAX+2];

	p = term(a, p);
	while (1) {
		if      (*p == '+') op = 1;
		else if (*p == '-') op = 0;
		else break;

		memset(t, 0, sizeof(t));
		p = term(t, p+1);

		add(a, t, op);
	}
	return p;
}

int _gcd(int a, int b)
{
	int r;
	while (b != 0) r = a % b, a = b, b = r;
	return a;
}

int norm(int *a)
{
	int	i, k = 0, g = 0;

	for (i = 0; i <= MAX; i++) if (a[i]) {
		k = i;
		if (!g) g = a[i];
		else g = _gcd(g, a[i]);
	}
	if (!g) return 0;
	if (g < 0) g = -g;
	if (a[k] < 0) g = -g;
	if (g != 1) for (i = 0; i <= k; i++) a[i] /= g;
	return k;
}

void mod(int *a, int *b)
{
	int i, ka, kb, m, g;

	ka = norm(a), kb = norm(b);
	while (ka >= kb) {
		g = _gcd(a[ka], b[kb]);
		if (g < 0) g = -g;
		m = b[kb] / g;
		for (i = 0; i <= ka; i++) a[i] *= m;
		m = a[ka] / b[kb];
		for (i = 0; i <= kb; i++) a[i+ka-kb] -= b[i] * m;
		while (ka > 0 && a[ka] == 0) ka--;
		if (ka == 0) break;
	}
	norm(a);
}

void gcd(int *a, int *b)
{
	int i;
	int t[MAX+2];

	for (i = MAX; i >= 0; i--) if (b[i]) break;
	if (i < 0) return;
	memcpy(t, a, sizeof(t)), memcpy(a, b, sizeof(t));
	mod(t, b);
	gcd(a, t);
}

int main()
{
	int i, k, f;
	char s[100];

	while (fgets(s, 100, stdin) && *s != '.') {
		memset(a, 0, sizeof(a));
		memset(b, 0, sizeof(b));

		parse(a, s);

		fgets(s, 100, stdin);
		parse(b, s);

		gcd(a, b);
		norm(a);

		f = 0;
		for (i = MAX; i >= 0; i--) {
			if (!a[i]) continue;
			if (a[i] > 0 && f) putchar('+');
			else if (a[i] < 0) putchar('-');
			f = 1;
			k = a[i]; if (k < 0) k = -k;
			if (i == 0) printf("%d", k);
			else {
				if (k > 1) printf("%d", k);
				putchar('x');
				if (i > 1) printf("^%d", i);
			}
		}
		if (!f) putchar('0');
		putchar('\n');
	}
	return 0;
}