// AOJ 2435: Zero Division Checker
// 2017.9.26 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define NUM 1
#define VAR 2

typedef struct { char nm[22]; int l, u; } T;
T var[102]; int m;

typedef struct { int id, n; char v[256]; } Q;	// n: ?????°??¨???v: ?????°??¨
Q q[102]; int top;

int main()
{
	int n, i, j, k;
	Q a, b, c;
	char bf[25];

	scanf("%d", &m);
	for (i = 0; i < m; i++)	scanf("%s%d%d", var[i].nm, &var[i].l, &var[i].u);

	scanf("%d", &n);
	top = 0;					// ????????????????????\??¬????????§???
	while (n-- > 0) {
		scanf("%s", bf);

		if (*bf == '+' || *bf == '-' || *bf == '*' || *bf == '/') {
			int a1, a2, b1, b2;
			if (top < 2) goto ERR;
			b = q[--top], a = q[--top];
			if (a.id == NUM) a1 = a2 = a.n, a.v[a1] = 1;
			else             a1 = 0, a2 = 255;
			if (b.id == NUM) b1 = b2 = b.n, b.v[b1] = 1;
			else             b1 = 0, b2 = 255;
			memset(c.v, 0, sizeof(c.v));

			for (i = a1; i <= a2; i++) if (a.v[i]) for (j = b1; j <= b2; j++) if (b.v[j]) {

				if      (*bf == '+') k = i + j;
				else if (*bf == '-') k = 256 + i - j;
				else if (*bf == '*') k = i * j;
				else { if (j == 0) goto ERR;		// '/'
									 k = i / j;
				}
				c.v[k & 0xff] = 1;
			}
			c.id = VAR;
			q[top++] = c;
		} else if (isdigit(*bf)) {
			q[top].id = NUM, q[top].n = atoi(bf) & 0xff;
			top++;
		} else {
			for (i = 0; i < m; i++)
				if (!strcmp(bf, var[i].nm)) break;
			if (i >= m) goto ERR;
			q[top].id = VAR;
			memset(q[top].v, 0, sizeof(q[top].v));
			for (j = var[i].l; j <= var[i].u; j++) q[top].v[j] = 1;
			top++;
		}
	}
	if (top == 1) puts("correct");
	else {
ERR:	puts("error");
	}
	return 0;
}