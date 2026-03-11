#include <stdio.h>
int min(int x, int y) { return x < y ? x : y; }
int main() {
	int a, b, n;
	scanf("%d%d%d", &n, &a, &b);
	printf("%d\n", min(a * n, b));
	return 0;
}