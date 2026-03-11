#include <stdio.h>

int main()
{
	int N, A, B, C, X, ans, cx, i;
	int Y[10001];
	while (scanf("%d%d%d%d%d", &N, &A, &B, &C, &X) && N) {
		ans = 0;
		for (int i = 0; i < N; i++)
			scanf("%d", &Y[i]);
		Y[N] = 10000;
		i = 0;
		while (N) {
			if (ans++ > 10000) {
				ans = 0;
				break;
			}
			if (X == Y[i])
				i++, N--;
			X = (A * X + B) % C;
		}
		printf("%d\n", ans - 1);
	}
	return 0;
}

