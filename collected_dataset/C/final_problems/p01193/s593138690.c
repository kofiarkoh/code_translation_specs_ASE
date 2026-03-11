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

int calc()
{
	int x, y, op;
	
	if ((x = factor()) == INF) return INF;
	while (1) {
		op = *p++;
		if (op == '=') break;
		if ((y = factor()) == INF) return INF;
		if      (op == '+') x += y;
		else if (op == '-') x -= y;
		else                x *= y;
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