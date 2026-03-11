#include <stdio.h>

#define Mod 1000000007

typedef struct List {
	struct List *next;
	int v;
} list;

int main()
{
	int i, j, N, M, S[2001], T[2001];
	scanf("%d %d", &N, &M);
	for (i = 1; i <= N; i++) scanf("%d", &(S[i]));
	for (j = 1; j <= M; j++) scanf("%d", &(T[j]));

	list *pos[100001] = {}, d[2001];
	for (j = 1; j <= M; j++) {
		d[j].v = j;
		d[j].next = pos[T[j]];
		pos[T[j]] = &(d[j]);
	}
	
	long long dp[2][2001];
	list *p;
	for (j = 0; j <= M; j++) dp[0][j] = 1;
	for (i = 1; i <= N; i++) {
		for (j = 0; j <= M; j++) dp[i%2][j] = dp[1-i%2][j];
		for (p = pos[S[i]]; p != NULL; p = p->next) {
			for (j = p->v; j <= M; j++) dp[i%2][j] += dp[1-i%2][p->v-1];
		}
		for (j = 0; j <= M; j++) dp[i%2][j] %= Mod;
	}
	
	printf("%lld\n", dp[N%2][M]);
	fflush(stdout);
	return 0;
}