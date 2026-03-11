#include <stdio.h>
#include <stdlib.h>

#define M 1000000007

int main()
{
	int N;
	char s[100002];
	char t[100002];
	int  cnt[128];
	int i;

	scanf("%d%s%s", &N, s, t);

	cnt[s[0]] = 1;
	for (N--, i = 1; i < N; i++) {
		cnt[s[i]] += cnt[t[i]];
		if (cnt[s[i]] >= M)
		{
		    cnt[s[i]] -= M;
		}
	}

	printf("%d\n", cnt[t[N]]);

	return 0;
}
