#include <stdio.h>

#define MAX 4

int
main(void)
{
	char N[MAX];
	scanf("%s", N);
	puts(N[0] == N[2] ? "Yes" : "No");
	return 0;
}
