// AOJ 1371: Infallibly Crack Perplexing Cryptarithm
// 2018.1.8 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define INF 0x7fffffff

int sz;
char raw[35], str[35], *p; int len;
char tr[128];
char token[9] = "()01+-*="; int tlen = 8;
char assign[8];
char used[8];
int ans;

int expr();

int factor()
{
	int x, f;

	f = 0; if (*p == '-') { while (*p == '-') p++, f = !f;	}
	if (*p == '(') {
		p++; if ((x = expr()) == INF) return INF;
		if (*p++ != ')') return INF;
	} else {
		x = 0;
		if (*p == '0') { p++; if (*p == '0' || *p == '1') return INF; }
		else if (*p == '1') {
			while (*p == '0' || *p == '1') x = (x << 1) + (*p++ & 1);
		} else return INF;
	}
	if (f) x = -x;
	return x;
}

int term()
{
	int x, y;

	if ((x = factor()) == INF) return INF;
	while (1) {
		if (*p != '*') break;
		p++;
		if ((y = factor()) == INF) return INF;
		x *= y;
	}
	return x;
}

int expr()
{
	int x, y, op;

	if ((x = term()) == INF) return INF;
	while (1) {
		if      (*p == '+') op = 1;
		else if (*p == '-') op = 0;
		else break;
		p++;
		if ((y = term()) == INF) return INF;
		if (op) x += y; else x -= y;
	}
	return x;
}

void calc()
{
	int par, eq, a, b;
	char *eq_p;

	memcpy(str, raw, len+1);
	par = 0, eq = 0;
	for (p = str; *p; p++) {
		if (isalpha(*p)) *p = assign[tr[*p]];
		if (*p == '=') {
			eq = 1, eq_p = p;
			if (par) return;
		}
		else if (*p == '(') par++;
		else if (*p == ')') par--;
	}
	if (!eq || par) return;
	if (*str == '=' || *(str+len-1) == '=') return;

	*eq_p++ = 0;
	p = str, a = expr();
	if (a == INF || *p != 0) return;
	p = eq_p, b = expr();
	if (b == INF || *p != 0) return;
	if (a == b) ans++;
}

void rec(int k)
{
	int i;

	if (k == sz) { calc(); return; }
	for (i = 0; i < tlen; i++) {
		if (used[i]) continue;
		used[i] = 1;
		assign[k] = token[i];
		rec(k+1);
		used[i] = 0;
	}
}

int main()
{
	int eq;

	eq = 0;
	memset(tr, -1, sizeof(tr));
	fgets(raw, 35, stdin);
	for (p = raw; *p > ' '; p++) {
		if (*p == '=') eq = 1;
		else if (isalpha(*p)) { if (tr[*p] < 0) tr[*p] = sz++; }
	}
	*p = 0, len = p-raw;

	if (sz + eq > 8) { puts("0"); return 0; }
	if (eq) tlen = 7;
	rec(0);
	printf("%d\n", ans);
	return 0;
}
