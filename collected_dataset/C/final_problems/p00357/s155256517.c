// Aizu 0362: Trampoline
// 2017.9.29 bal4u@uu

#include <stdio.h>

#define MAX 300005
int d[MAX+2];

int main()
{
	int n, i, j, a, k;

	scanf("%d", &n);
	for (i = 0; i < n; i++) scanf("%d", d+i);
	for (k = j = i = 0; i < n-1; i++) {
		a = j + d[i];
		if (a > k) k = a;
		j += 10;
		if (k < j) { puts("no"); return 0; }
	}
	for (k = j = (n-1)*10, i = n-1; i > 0; i--) {
		a = j - d[i];
		if (a < k) k = a;
		j -= 10;
		if (j < k) { puts("no"); return 0; }
	}
	puts("yes");
	return 0;
}