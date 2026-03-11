#include <stdio.h>

#define Mod 1000000007

long long div_mod(long long x, long long y, long long z)
{
	if (x % y == 0) return x / y;
	else return (div_mod((1 + x / y) * y - x, (z % y), y) * z + x) / y;
}

typedef struct List {
	struct List *next;
	int v, size, id;
	long long count;
} list;

list *adj[200001] = {}, e[400001];
long long fact[200001], fact_inv[200001];

long long recursion(int k)
{
	if (e[k^1].count > 0) return e[k^1].count;
	else e[k^1].count = 1;
	
	int u = e[k].v, w = e[k^1].v, tmp;
	list *p;
	for (p = adj[u]; p != NULL; p = p->next) {
		if (p->v == w) continue;
		recursion(p->id);
		e[k^1].size += e[(p->id)^1].size;
	}
	for (p = adj[u], tmp = e[k^1].size; p != NULL; p = p->next) {
		if (p->v == w) continue;
		e[k^1].count = e[k^1].count * e[(p->id)^1].count % Mod * fact[tmp] % Mod * fact_inv[e[(p->id)^1].size] % Mod * fact_inv[tmp - e[(p->id)^1].size] % Mod;
		tmp -= e[(p->id)^1].size;
	}
	e[k^1].size++;
	return e[k^1].count;
}

int main()
{
	int i, N, u, w;
	scanf("%d", &N);
	for (i = 0; i < N - 1; i++) {
		scanf("%d %d", &u, &w);
		e[i*2].v = w;
		e[i*2+1].v = u;
		e[i*2].count = 0;
		e[i*2+1].count = 0;
		e[i*2].size = 0;
		e[i*2+1].size = 0;
		e[i*2].id = i * 2;
		e[i*2+1].id = i * 2 + 1;
		e[i*2].next = adj[u];
		e[i*2+1].next = adj[w];
		adj[u] = &(e[i*2]);
		adj[w] = &(e[i*2+1]);
	}
	
	long long ans, tmp;
	list *p;
	for (i = 1, fact[0] = 1; i <= N; i++) fact[i] = fact[i-1] * i % Mod;
	for (i = N - 1, fact_inv[N] = div_mod(1, fact[N], Mod); i >= 0; i--) fact_inv[i] = fact_inv[i+1] * (i + 1) % Mod;
	for (i = 1; i <= N; i++) {
		for (p = adj[i], ans = 1, tmp = N - 1; p != NULL; p = p->next) {
			ans = ans * recursion(p->id) % Mod * fact[tmp] % Mod * fact_inv[e[(p->id)^1].size] % Mod * fact_inv[tmp - e[(p->id)^1].size] % Mod;
			tmp -= e[(p->id)^1].size;
		}
		printf("%lld\n", ans % Mod);
	}
	fflush(stdout);
	return 0;
}