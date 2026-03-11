#include <stdio.h>

int main()
{	
	int N, Q;
	char S[1000001];
	scanf("%d", &N);
	scanf("%s", S);
	scanf("%d", &Q);
	
	int i, k, q;
	long long ans, tmp[3];
	for (q = 1; q <= Q; q++) {
		scanf("%d", &k);
		ans = 0;
		tmp[0] = 0;
		tmp[1] = 0;
		tmp[2] = 0;
		
		for (i = 0; i < k; i++) {
			if (S[i] == 'D') tmp[0]++;
			else if (S[i] == 'M') {
				tmp[1]++;
				tmp[2] += tmp[0];
			} else if (S[i] == 'C') ans += tmp[2];
		}
		for (; S[i] != 0; i++) {
			if (S[i-k] == 'D') {
				tmp[0]--;
				tmp[2] -= tmp[1];
			} else if (S[i-k] == 'M') tmp[1]--;
			if (S[i] == 'D') tmp[0]++;
			else if (S[i] == 'M') {
				tmp[1]++;
				tmp[2] += tmp[0];
			} else if (S[i] == 'C') ans += tmp[2];
		}
		
		printf("%lld\n", ans);
	}
	
	fflush(stdout);
	return 0;
}