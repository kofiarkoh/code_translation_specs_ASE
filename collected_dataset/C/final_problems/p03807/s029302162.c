#include <stdio.h>

int main()
{
	int i, N, A, flag = 0;
	scanf("%d", &N);
	if (N == 1) {
		printf("YES\n");
		fflush(stdout);
		return 0;
	}
	for (i = 1; i <= N; i++) {
		scanf("%d", &A);
		flag ^= A & 1;
	}
	if (flag == 0) printf("YES\n");
	else printf("NO\n");
	fflush(stdout);
	return 0;
}