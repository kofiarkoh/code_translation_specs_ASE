// AOJ ITP2_1_D: Vector II
// 2018.6.24 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct { char id; int t; char x[13]; } BUF;
BUF buf[500005];
int qend[1002];
int *Q[1002];
char a[30];

int main()
{
	int n, q, i, j, t;
	char *p, *r;
	
	fgets(a, 30, stdin);
	n = atoi(a);
	p = a; while (*p >= '0') p++; q = atoi(p+1);
	for (i = 0; i < q; i++) {
		fgets(a, 30, stdin);
		buf[i].id = a[0];
		t = 0, p = a+2;
		while (*p >= '0') t = 10*t + (*p++ & 0xf);
		buf[i].t = t;
		if (a[0] == '0') {
			qend[t]++;
			p++, r = p+1;
			while (*r >= '0') r++; *r = 0;
			strcpy(buf[i].x, p);
		}
	}
	for (i = 0; i < n; i++) if (qend[i] > 0) {
		Q[i] = malloc((qend[i]+3)*sizeof(int)), qend[i] = 0;
	}
	for (i = 0; i < q; i++) {
		t = buf[i].t;
		if (buf[i].id == '0') Q[t][qend[t]++] = i; // pushBack
		else if (buf[i].id == '1') {
			if (qend[t] > 0) {    // dump
				printf(buf[Q[t][0]].x);
				for (j = 1; j < qend[t]; j++)
					putchar(' '), printf(buf[Q[t][j]].x);
			}
			putchar('\n');
		}
		else qend[t] = 0;         // clear
	}
	return 0;
}
