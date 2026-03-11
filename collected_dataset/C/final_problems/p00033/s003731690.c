#include <stdio.h>
int main() {
	int N, i, a, b, c;
	const char *result;
	scanf(" %d", &N);
	while (N--) {
		result = "YES";
		b = c = 0;
		for (i = 0; i < 10; i++) {
			scanf(" %d", &a);
			if (b < a) {
				b = a;
			} else if (c < a) {
				c = a;
			} else {
				result = "NO";
				break;
			}
		}
		puts(result);
	}
	return 0;
}