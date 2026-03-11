#include <stdio.h>

int main()
{
	int i, N, A[201], K[201];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d %d", &(A[i]), &(K[i]));
	
	int j, g;
	for (i = 1, g = 0; i <= N; i++) {
		if (K[i] == 1) g ^= A[i];
		else {
			for (j = A[i]; j >= K[i] && j % K[i] != 0; j -= j / K[i] + 1);
			if (j % K[i] == 0) g ^= j / K[i];
		}
	}
	
	if (g == 0) printf("Aoki\n");
	else printf("Takahashi\n");
	fflush(stdout);
	return 0;
}