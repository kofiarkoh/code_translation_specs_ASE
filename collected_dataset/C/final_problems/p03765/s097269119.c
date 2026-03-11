#include <stdio.h>

int main()
{
	int i, q, a, b, c, d;
	char S[100001], T[100001];
	scanf("%s", S);
	scanf("%s", T);
	scanf("%d", &q);
	
	int count[2][100001] = {};
	for (i = 0; S[i] != 0; i++) count[0][i+1] = count[0][i] + ((S[i] == 'A')? 1: 0);
	for (i = 0; T[i] != 0; i++) count[1][i+1] = count[1][i] + ((T[i] == 'A')? 1: 0);
	
	for (i = 1; i <= q; i++) {
		scanf("%d %d %d %d", &a, &b, &c, &d);
		if (((b - a + 1) - (count[0][b] - count[0][a-1]) * 2 + (count[1][d] - count[1][c-1]) * 2 - (d - c + 1)) % 3 == 0) printf("YES\n");
		else printf("NO\n");
	}
	fflush(stdout);
	return 0;
}