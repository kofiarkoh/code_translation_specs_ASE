#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int *par, *size, *height;
} UF_forest;

void UF_initialize(UF_forest *F, int n)
{
	int i;
	F->par = (int*)malloc(sizeof(int) * (n + 1));
	F->size = (int*)malloc(sizeof(int) * (n + 1));
	F->height = (int*)malloc(sizeof(int) * (n + 1));
	for (i = 1; i <= n; i++) {
		F->par[i] = i;
		F->size[i] = 1;
		F->height[i] = 1;
	}
}

void UF_merge(UF_forest *F, int u, int v)
{
	for (; F->par[u] != u; u = F->par[u]);
	for (; F->par[v] != v; v = F->par[v]);
	if (u == v) return;
	else if (F->height[u] > F->height[v]) {
		F->par[v] = u;
		F->size[u] += F->size[v];
	} else {
		F->par[u] = v;
		F->size[v] += F->size[u];
		if (F->height[u] == F->height[v]) F->height[v]++;
	}
}

int UF_root(UF_forest *F, int v)
{
	for (; F->par[v] != v; v = F->par[v]);
	return v;
}

int main()
{
	int N, Q;
	scanf("%d %d", &N, &Q);
	
	int i, t, u, w;
	UF_forest F;
	UF_initialize(&F, N);
	for (i = 1; i <= Q; i++) {
		scanf("%d %d %d", &t, &u, &w);
		if (t == 0) UF_merge(&F, u, w);
		else if (UF_root(&F, u) == UF_root(&F, w)) printf("1\n");
		else printf("0\n");
	}
	fflush(stdout);
	return 0;
}