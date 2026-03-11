#include <stdio.h>

int main(void) {
	char s[1999999], t[1999999];
	int count = 0;

	scanf("%s", s);
	scanf("%s", t);

	for (int i = 0; i < s[i] != '\0'; i++) {
		if (s[i] != t[i])
			count++;
	}

	printf("%d\n", count);

	return 0;
}