// Aizu Vol-10 1003: Extraordinary Girl II
// 2017.8.18 bal4u@uu

#include <stdio.h>
char *gets();

char tbl[10][8] = { { 0 },
{ '\'', ',', '.', '!', '?' },
{ 'a', 'b', 'c', 'A', 'B', 'C' },
{ 'd', 'e', 'f', 'D', 'E', 'F' },
{ 'g', 'h', 'i', 'G', 'H', 'I' },
{ 'j', 'k', 'l', 'J', 'K', 'L' },
{ 'm', 'n', 'o', 'M', 'N', 'O' },
{ 'P', 'q', 'r', 's', 'P', 'Q', 'R', 'S' },
{ 't', 'u', 'v', 'T', 'U', 'V' },
{ 'w', 'x', 'y', 'z', 'W', 'X', 'Y', 'Z' } };
int len[10] = { 0,5,6,6,6,6,6,8,6,8 };

char buf[10010], *p;

int main()
{
	int d, w;
	char c;

	while (gets(p = buf) != NULL) {
		while (*p) {
			c = *p++, d = c - '0';
			w = 0;  while (*p == c) w++, p++;
			if (d == 0) while (w-- > 0) putchar(' ');
			else putchar(tbl[d][w % len[d]]);
		}
		putchar('\n');
	}
	return 0;
}