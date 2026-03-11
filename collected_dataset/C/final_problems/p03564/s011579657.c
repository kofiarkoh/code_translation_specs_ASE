#include <stdio.h>

int main () {
	int n, k, a = 1;
	scanf("%d %d", &n, &k);
	
	for (int i = 1; i <= n; i++) {
		if ( a * 2 < a + k) {
			a *= 2;
		}
		else {
			a += k;
		}
	}
	printf("%d", a);
}