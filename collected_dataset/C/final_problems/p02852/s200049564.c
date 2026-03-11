#include <stdio.h>

int main()
{
	int i, N, M;
	char S[100002];
	scanf("%d %d", &N, &M);
	scanf("%s", S);
	
	int j, k, dice[100001];
	for (i = N, k = 0; i > 0; ) {
		for (j = (i < M)? i: M; j >= 1; j--) if (S[i-j] == '0') break;
		if (j == 0) break;
		dice[k++] = j;
		i -= j;
	}
	
	if (i > 0) printf("-1\n");
	else {
		for (i = k - 1; i >= 1; i--) printf("%d ", dice[i]);
		printf("%d\n", dice[0]);
	}
	fflush(stdout);
	return 0;
}