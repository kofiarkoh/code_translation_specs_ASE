#include <stdio.h>

#define A 4800

int main() {
	static char used[16];
	int n, i, k, c, cnt;

	scanf("%d", &n);
	k = 0;
	for (i = 0; i < n; i++) {
		int a;

		scanf("%d", &a);
		if (a < 3200)
			used[a / 400] = 1;
		else
			k++;
	}
	cnt = 0;
	for (c = 0; c < 8; c++)
		if (used[c])
			cnt++;
	if (cnt == 0)
		printf("1 %d\n", n);
	else
		printf("%d %d\n", cnt, cnt + k);
	return 0;
}
