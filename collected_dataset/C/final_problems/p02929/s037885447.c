#include <stdio.h>

#define Mod 1000000007

int main()
{
	int N;
	char S[200001];
	scanf("%d", &N);
	scanf("%s", S);
	
	int i, count = 0;
	long long ans = 1;
	for (i = 0; S[i] != 0; i++) {
		if (i % 2 == 0 && S[i] == 'B' || i % 2 == 1 && S[i] == 'W') count++;
		else if (count > 0) ans = ans * (count--) % Mod;
		else {
			printf("0\n");
			fflush(stdout);
			return 0;
		}
	}
	for (i = 2; i <= N; i++) ans = ans * i % Mod;
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}