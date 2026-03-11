// Aizu Vol-3 0350: Irreducible Fractionalization
// 2017.8.26 bal4u@uu

#include <stdio.h>
#include <ctype.h>

char *gets(char *);
char buf[100], *p;

long long gcd(long long a, long long b)
{
	long long r;
	while (b != 0) r = a % b, a = b, b = r;
	return a;
}

int main()
{
	int A, B, C, D, E;		// A + B/C + D/E --> a/b
	long long a, b, g;

	gets(p = buf);
	A = 0; while (isdigit(*p)) A = 10 * A + (*p++ - '0');
	p++;   // if (*p == '.') p++;
	B = 0, C = 1; while (isdigit(*p)) B = 10 * B + (*p++ - '0'), C *= 10;
	D = 0, E = 1;
	if (*p == '(') {
		p++, E = 1;
		while (isdigit(*p)) D = 10 * D + (*p++ - '0'), E *= 10;
		E--, E *= C;
	}
	b = C, b *= E;
	a = A * b, a += (long long)B * E, a += (long long)D * C;
	g = gcd(a, b), a /= g, b /= g;
	printf("%lld/%lld\n", a, b);
	return 0;
}