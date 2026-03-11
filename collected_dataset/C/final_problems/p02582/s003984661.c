#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>
#include <fcntl.h>

int main()
{
	char str[3];

	scanf("%s", str);
	if (!(strcmp(str, "RRR")))
		printf("3");
	else if (!(strcmp(str, "RRS")))
		printf("2");
	else if (!(strcmp(str, "SRR")))
		printf("2");
	else if (!(strcmp(str, "RSR")))
		printf("1");
	else if (!(strcmp(str, "SRS")))
		printf("1");
	else if (!(strcmp(str, "RSS")))
		printf("1");
	else if (!(strcmp(str, "SSR")))
		printf("1");
	else
		printf("0");
	return (0);
}
