// AOJ 2731 Shifting a Matrix
// 2018.2.11 bal4u

#include <stdio.h>
#include <ctype.h>

#define SIZE 1000

int N;
typedef struct { short r, c; } T;
T tbl[SIZE+2][101][101];
char s[1002], *p;
char used[SIZE+2];

char *seq(int res, char *s);

// malloc
int new() {	int i;
	for (i = 0; i < SIZE; i++) if (!used[i]) { used[i] = 1; break; }
	return i;
}

// free
void dis(int k) { used[k] = 0; }

char *in(int *res, char *p) { int n = 0;
	while (isdigit(*p)) n = (n<<3)+(n<<1) + (*p++ & 0xf); *res = n; return p;
}

void calc(int a, int b)
{
	int r, c;
	int tmp;

	tmp = new();
	for (r = 0; r < N; r++) for (c = 0; c < N; c++) {
		tbl[tmp][r][c] = tbl[a][tbl[b][r][c].r][tbl[b][r][c].c];
	}
	for (r = 0; r < N; r++) for (c = 0; c < N; c++) tbl[a][r][c] = tbl[tmp][r][c];
	dis(tmp);
}

char *ope(int a, char *s)
{
	int k, r, c;
	char x;

	x = *s++;
	s = in(&k, s), k--;
	for (r = 0; r < N; r++) for (c = 0; c < N; c++)
		tbl[a][r][c].r = r, tbl[a][r][c].c = c;

	if      (x == 'L') for (c = 0; c < N; c++) tbl[a][k][c].c = (c+1  ) % N;
    else if (x == 'R') for (c = 0; c < N; c++) tbl[a][k][c].c = (c-1+N) % N;
    else if (x == 'U') for (r = 0; r < N; r++) tbl[a][r][k].r = (r+1  ) % N;
	else               for (r = 0; r < N; r++) tbl[a][r][k].r = (r-1+N) % N;
	return s;
}

char *rep(int a, char *s)
{
	int n, r, c;
	int b;
	
	b = new();
	s = seq(b, s)+1;
	s = in(&n, s);
	for (r = 0; r < N; r++) for (c = 0; c < N; c++)
		tbl[a][r][c].r = r, tbl[a][r][c].c = c;

	while (n) {
		if (n & 1) calc(a, b);
		calc(b, b);
		n >>= 1;
	}
	dis(b);
	return s;
}

char *seq(int res, char *s)
{
	int r, c;
	int tmp;

	for (r = 0; r < N; r++) for (c = 0; c < N; c++)
		tbl[res][r][c].r = r, tbl[res][r][c].c = c;
	tmp = new();
	while (*s == '(' || isalpha(*s)) {
		if (*s == '(') s = rep(tmp, s + 1);
		else s = ope(tmp, s);
		calc(res, tmp);
	}
	dis(tmp);
	return s;
}

int main()
{
	int L, r, c;
	int ans;
	
	fgets(p=s, 16, stdin), p = in(&N, p), in(&L, p+1);
	fgets(s, 1002, stdin);

	ans = new();
	seq(ans, s);
	for (r = 0; r < N; r++) {
		printf("%d", N*tbl[ans][r][0].r + tbl[ans][r][0].c + 1);
		for (c = 1; c < N; c++)
			printf(" %d", N*tbl[ans][r][c].r + tbl[ans][r][c].c + 1);
		putchar('\n');
	}
	return 0;
}
