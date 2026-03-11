#include<stdio.h>
#include<stdlib.h>

int main(void) {
	int N, M, x, ans = 0;
	int a;
	
	while (1) {
		scanf("%d %d", &N, &M);
		if (N == 0 && M == 0) {
			break;
		}
		x = M / N;
		for (int i = 0; i < N; i++) {
			scanf("%d", &a);
			if (x > a) ans += a;
			else ans += x;
		}
		printf("%d\n", ans);
		ans = 0;
	}

	return 0;
}


