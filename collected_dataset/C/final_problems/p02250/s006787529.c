// AOJ ALDS1_14_D Multiple String Matching
// 2018.5.9 bal4u

#include <stdio.h>
//#include <stdlib.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif

int in()
{
	int n = 0, c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

char text[1000005];
char pattern[10005];
int  next[10005], skip[260], g[10005];

int bmMatch(char *text, int len, char *pattern, int patlen,
              int *skip, int *next)
{
    int i, j;

    i = patlen - 1;
    while (i < len) {
        j = patlen - 1;
        while (j >= 0 && text[i] == pattern[j]) i--, j--;
        if (j < 0) return 1;
        if (skip[text[i]] >= next[j]) i += skip[text[i]];
        else i += next[j];
    }
    return 0;
}

int bmSkip(int *skip, char *pattern, int patlen)
{
    int j;

    for (j = 0; j < 256; j++) skip[j] = patlen;
    for (j = 0; j < patlen - 1; j++) skip[pattern[j]] = patlen-1-j;
    return 0;
}

int bmNext(int *next, int nlen, char *pattern, int patlen)
{
    int  j, k, s;

    for (j = 0; j < patlen; j++) next[j] = 2*patlen - 1 - j;
    j = patlen;
    for (k = patlen - 1; k >= 0; k--) {
        g[k] = j;
        while (j != patlen && pattern[j] != pattern[k]) {
            next[j] = (next[j] <= patlen-1-k) ? next[j] : patlen-1-k;
            j = g[j];
        }
        j--;
    }
    s = j;
    for (j = 0; j < patlen; j++) {
        next[j] = (next[j] <= s+patlen-j) ? next[j] : s+patlen-j;
        if (j >= s) s = g[s];
    }
    return 0;
}

int main()
{
	int Q, c, tlen, plen;
	char *p;

	p = text;
	while ((c = gc()) > ' ') *p++ = c;
	*p = 0; tlen = p-text;

	Q = in();
	while (Q--) {
		p = pattern;
		while ((c = gc()) > ' ') *p++ = c;
		*p = 0; plen = p-pattern;
	
	    bmSkip(skip, pattern, plen);
		bmNext(next, plen, pattern, plen);
 		pc('0' + bmMatch(text, tlen, pattern, plen, skip, next));
		pc('\n');
	}
	return 0;
}
