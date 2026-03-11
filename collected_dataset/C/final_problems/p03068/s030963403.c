#include <stdio.h>
int main() {
	int k, n;
	char s[11];
	scanf("%d%s%d", &n, s, &k);
	for (int i = 0; i < n; i++) {
		if (s[k - 1] != s[i]) {
			printf("*");
		} else {
			printf("%c", s[i]);
		}
	}
	printf("\n");
	return 0;
}