// AOJ 0330: Halting Problem
// 2017.10.27 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ADDV	1
#define ADDC	2
#define SUBV	3
#define SUBC	4
#define SETV	5
#define SETC	6
#define IFV		7
#define HALT	8

typedef struct { int lin, cmd, v1, v2, v3, con, to; } T;
T pgm[52]; int n;

int tr['z'+1];
char var[26]; int sz;
int lin[1002];
char tmp[110][26];
char mk[51][16][16][16][16][16];

void doprint()
{
	int i;
	for (i = 'a'; i <= 'z'; i++) if (tr[i] >= 0) printf("%c=%d\n", i, var[tr[i]]);
}

void calc(int id, int v)
{
	if (v < 0 || v >= 16) { doprint(); exit(0); }
	var[id] = v;
}

int varid(char x)
{
	int r;
	if (tr[x] < 0) r = sz++, tr[x] = r;
	else r = tr[x];
	return r;
}

int main()
{
	int i, pc, cnt;
	char cmd, b1[10], b2[10], b3[10];
	T *tp;

	memset(tr, -1, sizeof(tr)), memset(lin, -1, sizeof(lin)), sz = 0;
	scanf("%d", &n);
	for (tp = pgm, i = 0; i < n; i++, tp++) {
		scanf("%d%s", &tp->lin, b1); lin[tp->lin] = i, cmd = *(b1+1);
		if (cmd == 'A') tp->cmd = HALT;
		else if (cmd == 'D' || cmd == 'U') {
			scanf("%s%s%s", b1, b2, b3);
			tp->v1 = varid(*b1), tp->v2 = varid(*b2);
			if (*b3 >= 'a') {
				tp->cmd = (cmd == 'D') ? ADDV : SUBV;
				tp->v3 = varid(*b3);
			} else {
				tp->cmd = (cmd == 'D') ? ADDC : SUBC;
				tp->con = atoi(b3);
			}
		} else if (cmd == 'E') {
			scanf("%s%s", b1, b2);
			tp->v1 = varid(*b1);
			if (*b2 >= 'a') {
				tp->cmd = SETV;
				tp->v2 = varid(*b2);
			} else {
				tp->cmd = SETC;
				tp->con = atoi(b2);
			}
		} else {		// IF
			scanf("%s%d", b1, &tp->to);
			tp->cmd = IFV, tp->v1 = varid(*b1);
		}
	}
	pc = 0, cnt = 0;
	while (1) {
		tp = pgm+pc;
		if      (tp->cmd == ADDV) calc(tp->v1, var[tp->v2] + var[tp->v3]);
		else if (tp->cmd == ADDC) calc(tp->v1, var[tp->v2] + tp->con);
		else if (tp->cmd == SUBV) calc(tp->v1, var[tp->v2] - var[tp->v3]);
		else if (tp->cmd == SUBC) calc(tp->v1, var[tp->v2] - tp->con);
		else if (tp->cmd == SETV) calc(tp->v1, var[tp->v2]);
		else if (tp->cmd == SETC) calc(tp->v1, tp->con);
		else if (tp->cmd == HALT) break;

		if (tp->cmd == IFV && var[tp->v1] != 0) {
			if (lin[tp->to] < 0) break;
			if (mk[pc][var[0]][var[1]][var[2]][var[3]][var[4]]) { puts("inf"); return 0; }
			mk[pc][var[0]][var[1]][var[2]][var[3]][var[4]] = 1;
			pc = lin[tp->to];
		} else {
			pc++;
			if (pc == n) break;
		}
	}
	doprint();
	return 0;
}