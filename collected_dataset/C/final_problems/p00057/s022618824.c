#include <stdio.h>
#include <math.h>
int f(int n) {
	if (!n) return 1;
	return f(n-1)+n;
}
int main() {
	int n;
	while(scanf("%d", &n) != EOF) {
		printf("%d\n", f(n));
	}
	return 0;
}