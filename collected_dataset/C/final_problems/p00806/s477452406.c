// AOJ 1227: 77377
// 2017.10.17 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 500

int n;
char word[102][52]; int len[102];
char seq[303];
char ans[303];

char tr['z'];

void parse(char *ans, int w, char *s, int sz)
{
	int i, j, k;
	char *p, *q, *u, *v;

	for (i = 0; i < n; i++) if ((j = len[i]) <= sz) {
		p = word[i], q = s;
		while (*p && *q && tr[*p] == *q) p++, q++;
		if (*p) continue;
		if (*q) {
			u = v = malloc(MAX), k = w;
			if (k > 0) {
				memcpy(v, ans, k); v += k;
				*v++ = ' ', k++;
			}
			strcpy(v, word[i]);
			parse(u, k+j, s+j, sz-j);
			free(u);
		} else {
			v = ans + w; if (w > 0) *v++ = ' ';
			strcpy(v, word[i]), v += j;
			*v++ = '.', *v = 0;
			puts(ans);
		}
	}
}

int main()
{
	int i;

	tr['a']=tr['b']=tr['c'] = '2'; tr['d']=tr['e']=tr['f'] = '3';
	tr['g']=tr['h']=tr['i'] = '4'; tr['j']=tr['k']=tr['l'] = '5';
	tr['m']=tr['n']=tr['o'] = '6'; tr['p']=tr['q']=tr['r']=tr['s'] = '7';
	tr['t']=tr['u']=tr['v'] = '8'; tr['w']=tr['x']=tr['y']=tr['z'] = '9';
	while (scanf("%d", &n) && n > 0) {
		for (i = 0; i < n; i++) scanf("%s", word[i]), len[i] = strlen(word[i]);
		scanf("%s", seq);
		parse(ans, 0, seq, strlen(seq));
		puts("--");
	}
	return 0;
}