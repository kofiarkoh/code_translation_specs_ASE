// Aizu Vol0 0057: The Number of Area
// 2017.7.29

#include <stdio.h>
#include <stdlib.h>

char *gets(char *);
char buf[100];

int main()
{
	int n;

	while (gets(buf) != NULL) {
		n = atoi(buf);
		printf("%d\n", 1 + n*(n + 1) / 2);
	}
	return 0;
}