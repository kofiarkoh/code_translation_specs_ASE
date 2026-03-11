// AOJ 2730: Line Gimmick
// 2017.10.27 bal4u@uu
#include <stdio.h>
#include <stdlib.h>

char buf[100002];

int main()
{
	int n, i, j;

	fgets(buf, 20, stdin); n = atoi(buf);
	fgets(buf, 100002, stdin);
	i = 0;   while (buf[i] == '<') i++;
	j = n-1; while (buf[j] == '>') j--;
	if (i > n-1-j) i = n-1-j;
	printf("%d\n", n-i);
	return 0;
}