// Aizu 1102:   Calculation of Expressions
// 2017.9.12 bal4u@uu

#include <stdio.h>
#include <ctype.h>

#define abs(a) ((a)>=0?(a):-(a))
#define OVER 10000

char buf[200], *p;

int factor(int *r, int *i)
{
	int r1, i1, n;
	int calc();

	if (*p == '(') { p++; if (!calc(&r1, &i1)) return 0; p++; }
	else {
		r1 = i1 = n = 0;
		while (isdigit(*p)) {
			n = 10 * n + (*p++ - '0'); if (n > OVER) return 0;
		}
		if (*p == '*' && *(p+1) == 'i') i1 = n, p+=2;
		else r1 = n;
	}
	*r = r1, *i = i1;
	return 1;
}

int term(int *r, int *i)
{
	int r1, r2, i1, i2, t;
	long long t1, t2;

	if (!factor(&r1, &i1)) return 0;
	while (1) {
		if (*p == '*') p++;
		else break;
		if (!factor(&r2, &i2)) return 0;

//		rr = r1*r2-i1*i2, ii = r1*i2+i1*r2;
		t1 = (long long)r1 * (long long)r2, t2 = (long long)i1 * (long long)i2,	t1 -= t2;
		if (abs(t1) > 10000ULL) return 0;
		t = (int)t1;
		t1 = (long long)r1 * (long long)i2, t2 = (long long)i1 * (long long)r2,	t1 += t2;
		if (abs(t1) > 10000ULL) return 0;
		r1 = t, i1 = (int)t1;
	}
	*r = r1, *i = i1;
	return 1;
}

int calc(int *r, int *i)
{
	int r1, r2, i1, i2, op;
	
	if (!term(&r1, &i1)) return 0;
	while (1) {
		if      (*p == '+') op = 1;
		else if (*p == '-') op = 0;
		else break;
		p++;
		if (!term(&r2, &i2)) return 0;
		if (op) r1 += r2, i1 += i2;
		else    r1 -= r2, i1 -= i2;
		if (abs(r1) > OVER || abs(i1) > OVER) return 0;
	}
	*r = r1, *i = i1;
	return 1;
}

int main()
{
	int r, i;

	while (fgets(p = buf, 150, stdin) != NULL) {
		if (!calc(&r, &i)) puts("overflow");
		else {
			if (r == 0 && i == 0) puts("0");
			else if (i == 0) printf("%d\n", r);
			else if (r == 0) printf("%di\n", i);
			else printf("%d%+di\n", r, i);
		}
	}
	return 0;
}