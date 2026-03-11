#include <stdio.h>

int main()
{
	int i, j, N;
	char S[601][301];
	scanf("%d", &N);
	for (i = 0; i < N; i++) {
		scanf("%s", S[i]);
		for (j = 0; j <= N; j++) S[i+N][j] = S[i][j];
	}
	
	int k, count = 0;
	for (k = 0; k < N; k++) {
		for (i = 0; i < N; i++) {
			for (j = 0; j < N; j++) {
				if (S[i+k][j] != S[j+k][i]) break;
			}
			if (j < N) break;
		}
		if (i == N) count++;
	}
	
	printf("%d\n", count * N);
	fflush(stdout);
	return 0;
}