// AOJ 2109 Ancient Expression
// 2018.3.5 bal4u

#include <stdio.h>
#include <string.h>
#include <ctype.h>

int  G;
int  rank[128], ass[12];		// ass 0:left to right, 1:right to left
char gb[12][55], gw[12];

int  top, end;
char q[101], s[101];
char sp[101][202]; int len[101];

char buf[105], *p;
int in()
{
	int n = 0;
	while (*p >= '0') n = 10*n + (*p++ & 0xf); p++;
	return n;
}

void pushback(int min)
{
	int i, g;
	char op;
	
	if (!top) return;
	while (top && s[top-1] != '(') {
		op = s[top-1];
		if ((g = rank[op]) <= min) break;
		gb[g][gw[g]++] = op, top--;
	}
	for (g = G-1; g > min; g--) {
		if (!gw[g]) continue;
		if (ass[g]) for (i = 0; i < gw[g]; i++)    q[end++] = gb[g][i];
		else        for (i = gw[g]-1; i >= 0; i--) q[end++] = gb[g][i];
		gw[g] = 0;
	}
}

void parse()
{
	int g;

	p = buf, top = 0, end = 0;
	while (1) {
		if (*p <= ' ') { pushback(0); break; }
		if (isalpha(*p)) q[end++] = *p;
		else if (*p == ')') pushback(0), top--;
		else if (*p == '(') s[top++] = '(';
		else {
			g = rank[*p];
			if (ass[g]) pushback(g);
			else        pushback(g-1);
			s[top++] = *p;
		}
		p++;
	}
}

void cout()
{
	int s;
	char c, t[250];

	s = 0, top = 0;
	while (top < end) {
		if (isalpha(c=q[top])) sp[s][0] = c, sp[s][1] = 0, len[s] = 1;
		else {
			s -= 2; p = t;
			*p++ = '(', memcpy(p, sp[s  ], len[s  ]), p += len[s  ];
			*p++ = c,   memcpy(p, sp[s+1], len[s+1]), p += len[s+1];
			*p++ = ')', *p = 0;
			len[s] = p-t, memcpy(sp[s], t, p-t+1);
		}
		s++, top++;
	}
	puts(sp[0]);
}

int main()
{
	int M, N, g;
	int cno, cmax;

	fgets(p=buf, 10, stdin), cmax = in();
	for (cno = 0; cno < cmax; cno++) {
		if (cno) putchar('\n');

		fgets(p=buf, 10, stdin), G = in()+1;
		for (g = 1; g < G; g++) {
			fgets(p=buf, 100, stdin);
			ass[g] = (*p == 'R'), p+=2;
			M = in(); while (M--) rank[*p] = g, p+=2;
		}

		fgets(p=buf, 10, stdin), N = in();
		while (N--) {
			fgets(buf, sizeof(buf), stdin);
			parse();
			cout();
		}
	}
	return 0;
}
