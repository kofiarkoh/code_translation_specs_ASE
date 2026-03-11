#include <stdio.h>
#include <stdlib.h>

#define Mod 1000000007

typedef struct List {
	struct List *next;
	int v;
} list;

int main()
{
	int i, N, P[5001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", &(P[i]));
	
	list **adj = (list**)malloc(sizeof(list*) * (N + 1)), *d = (list*)malloc(sizeof(list) * N * 2);
	for (i = 0; i < N; i++) {
		if (P[i+1] == -1) continue;
		d[i*2].v = P[i+1];
		d[i*2+1].v = i + 1;
		d[i*2].next = adj[i+1];
		d[i*2+1].next = adj[P[i+1]];
		adj[i+1] = &(d[i*2]);
		adj[P[i+1]] = &(d[i*2+1]);
	}
	
	list *p;
	int j, k, K, comp[5001] = {}, q[5001], head, tail, size[5001] = {};
	for (i = 1, k = 1; i <= N; i++) {
		if (P[i] != -1) continue;
		comp[i] = k;
		q[0] = i;
		for (head = 0, tail = 1; head < tail; head++) {
			for (p = adj[q[head]]; p != NULL; p = p->next) {
				if (comp[p->v] == 0) {
					comp[p->v] = k;
					q[tail++] = p->v;
				}
			}
		}
		size[k++] = tail;
	}
	for (i = 1, K = k - 1; i <= N; i++) {
		if (comp[i] != 0) continue;
		comp[i] = k;
		q[0] = i;
		for (head = 0, tail = 1; head < tail; head++) {
			for (p = adj[q[head]]; p != NULL; p = p->next) {
				if (comp[p->v] == 0) {
					comp[p->v] = k;
					q[tail++] = p->v;
				}
			}
		}
		k++;
	}
	free(adj);
	free(d);
	
	int l;
	long long ans = N - k + 1 + K, dp[2][5001] = {{1}}, N_pow[5001] = {1}, fact;
	for (i = 1; i <= K; i++) {
		l = i % 2;
		N_pow[i] = N_pow[i-1] * (N - 1) % Mod;
		dp[l][0] = dp[1-l][0] * size[i] % Mod;
		for (j = 1; j <= i; j++) dp[l][j] = (dp[1-l][j-1] + dp[1-l][j] * size[i]) % Mod;
	}
	ans = (ans * N_pow[K] - (dp[K%2][K-1] - K) * N_pow[K-1] % Mod) % Mod;
	for (i = 2, fact = 1; i <= K; i++, fact = fact * (i - 1) % Mod) {
		ans = (ans + Mod - ((dp[K%2][K-i] * fact % Mod) * N_pow[K-i] % Mod)) % Mod;
	}
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}