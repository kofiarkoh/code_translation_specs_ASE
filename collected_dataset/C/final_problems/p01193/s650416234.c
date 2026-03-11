// Aizu 2070: First Experience
// 2017.9.22 bal4u@uu

#include <stdio.h>
#include <ctype.h>

#define INF 0x7fffffff

char buf[100], *p;

int factor()
{
	int x = 0;
	while (isdigit(*p)) { x = x * 10 + *p++ - '0'; if (x > 9999) return INF; }
	return x;
}

int term()
{
	int x, y, op;

	if ((x = factor()) == INF) return INF;
	while (1) {
		if (*p == '*') op = 1;
		else break;
		p++;
		if ((y = factor()) == INF) return INF;
		if (op) x *= y;
		if (x < 0 || x > 9999) return INF;
	}
	return x;
}

int calc()
{
	int x, y, op;
	
	if ((x = term()) == INF) return INF;
	while (1) {
		if      (*p == '+') op = 1;
		else if (*p == '-') op = 0;
		else if (*p == '=') return x;
		else break;
		p++;
		if ((y = term()) == INF) return INF;
		if (op) x += y; else x -= y;
		if (x < 0 || x > 9999) return INF;
	}
	return x;
}

int main()
{
	int ans;

	while (scanf("%s", p = buf) == 1) {
		if ((ans = calc()) != INF) printf("%d\n", ans);
		else puts("E");
	}
	return 0;
}