#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define min(a, b) a < b ? a : b
#define max(a, b) a > b ? a : b

#define MAXS 200000

int main(void)
{
	int i, j, ans = 0;
	char s[MAXS + 1];

	scanf("%s", s);

	for (i = 0; i < strlen(s); i++) {
		if (s[i] == 'A') {
			break;
		}
	}
	for (j = strlen(s) - 1; j >= 0; j--) {
		if (s[j] == 'Z') {
			break;
		}
	}
	ans = j - i + 1;

	printf("%d\n", ans);

	return (0);
}