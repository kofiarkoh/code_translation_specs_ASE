#include <stdio.h>
#include <string.h>

int main() {
	static char s[4];

	scanf("%s", s);
	if (strcmp(s, "SUN") == 0)
		printf("7\n");
	else if (strcmp(s, "MON") == 0)
		printf("6\n");
	else if (strcmp(s, "TUE") == 0)
		printf("5\n");
	else if (strcmp(s, "WED") == 0)
		printf("4\n");
	else if (strcmp(s, "THU") == 0)
		printf("3\n");
	else if (strcmp(s, "FRI") == 0)
		printf("2\n");
	else
		printf("1\n");
	return 0;
}
