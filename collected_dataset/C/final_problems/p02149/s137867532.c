// AOJ 3051 Lunch
// 2018.3.16 bal4u

#include <stdio.h>
#include <stdlib.h>

int main()
{
	int max, a, i, id;

	max = 0; for (i = 0; i < 3; i++) {
		scanf("%d", &a);
		if (a > max) max = a, id = i;
	}
	printf("%c\n", 'A' + id);
	return 0;
}
