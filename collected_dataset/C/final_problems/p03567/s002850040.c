#include <stdio.h>
#include <string.h>

int main(void) {
	char s[6];

	scanf("%s", s);

	printf("%s\n", strstr(s, "AC") ? "Yes" : "No");

	return 0;
}
