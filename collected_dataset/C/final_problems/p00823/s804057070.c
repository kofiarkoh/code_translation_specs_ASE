// AOJ 1244: Molecular Formula
// 2017.10.12 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

int w[1024];

void insert(char *s, int n)
{
	int i;
	i = (s[0]-'A'+1);
	if (s[1]) i |= (s[1]-'a'+1) << 5;
	w[i] = n;
}

int lookup(char *s)
{
	int i;
	i = (s[0]-'A'+1);
	if (s[1]) i |= (s[1]-'a'+1) << 5;
	return w[i];
}

char *molecular();
char *atom(int *n, char *s)
{
	int k;
	char b[3];

	if (*s == '(') {
		s++; if ((s = molecular(n, s)) == NULL) return NULL;
		s++;
		if (isdigit(*s)) {
			k = 0; while (isdigit(*s)) k = 10*k + (*s++ - '0');
			*n *= k;
		}
	} else if (isalpha(*s)) {
		b[0] = *s++, b[1] = 0;
		if (islower(*s)) b[1] = *s++, b[2] = 0;
		if ((k = lookup(b)) == 0) return NULL;
		*n = k;
	}
	return s;
}

char *molecular(int *n, char *s)
{
	int k, m;

	*n = 0, m = 0;
	while (*s >= ' ') {
		if (*s == '(' || isalpha(*s)) {
			if ((s = atom(&m, s)) == NULL) return NULL;
			k = 1;
		} else if (*s == ')') break;
		if (isdigit(*s)) {
			k = 0; while (isdigit(*s)) k = 10*k + (*s++ - '0');
		}
		*n += m*k;
	}
	return s;
}

int main()
{
	int ans;
	char buf[202], *p, *q;

	while (fgets(buf, 200, stdin) && *(buf+2) != 'D') {
		p = buf;
		while (isspace(*p)) p++;
		q = p; while (isalpha(*p)) p++;
		*p++ = 0;
		insert(q, atoi(p));
	}

	while (fgets(buf, 200, stdin) && *buf != '0') {
		p = buf; while (isspace(*p)) p++;
		if (molecular(&ans, p) == NULL) puts("UNKNOWN");
		else printf("%d\n", ans);
	}
	return 0;
}