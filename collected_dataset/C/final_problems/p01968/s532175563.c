// AOJ 2868 Hierarchical Calculator
// 2018.3.29 bal4u

#include <stdio.h>

int a[63];
int c[5];		// a = -2, -1, 0, 1, 2;

int main()
{
	int N, i;

	scanf("%d", &N);
	for (i = 1; i <= N; i++) {
		scanf("%d", a+i);
		c[2+a[i]]++;
	}
	c[2] = c[3] = 0;
	if (!(c[0] & 1)) c[1] = 0;
	else if (c[1]) c[1] = 1;
	else c[0]--;
	printf("%d\n", c[0]+c[1]+c[4]);
	for (i = 1; i <= N; i++) {
		if (c[2+a[i]]) printf("%d\n", i), c[2+a[i]]--;
	}
	return 0;
}
