// AOJ 2570: Shipura
// 2017.12.1 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

#define M  1000000007
#define TOP -1
#define SFT -2
#define S   -3

char b1[2000002];
int stack[2000002], top;

int main()
{
	int x, y;
	char *p;

	while (fgets(b1+1, sizeof(b1), stdin) && *(b1+1) != '#') {
		p = b1, *p++ = 0; while (*p >= ' ') *p++; *p-- = 0;
		while (*p) {
			if (*p == '<') { *p-- = ' '; continue; }
			if (!isdigit(*p) && *p != 'S') { p--; continue; }
			if (*p == 'S') p--;
			else while (isdigit(*p)) p--;
			while (*p == ' ') p--;
			if (*p == '>' && *(p-1) == '>') *p = 'R', *(p-1) = ' ', p-=2;
		}

		p++, stack[0] = TOP, top = 1;
		while (*p) {
			while (*p == ' ') p++;
			if (*p == 'S') { p++, stack[top++] = S; continue; }
			if (*p == 'R') { p++, stack[top++] = SFT; continue; }
			if (isdigit(*p)) {
				x = 0; while (isdigit(*p)) x = (x<<3)+(x<<1) + (*p++ & 0xf);
			} else {		//  '>'
				p++;
				x = stack[--top];
				x = (long long)x * x % M;
				top--;
			}
			if (stack[top-1] == SFT) {
				top -= 2;
				y = stack[top];
				if (x > 31) y = 0;
				else y >>= x;
				x = y;
			}
			stack[top++] = x;
		}
		printf("%d\n", x);
	}
	return 0;
}