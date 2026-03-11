#include <stdio.h>
#include <stdlib.h>

typedef struct List {
	struct List *next;
	int v;
} list;

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

int UF_size(UF_forest *F, int v)
{
	for (; F->par[v] != v; v = F->par[v]);
	return F->size[v];
}

int main()
{
	int i, N, x[100001], y[100001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d %d", &(x[i]), &(y[i]));

	list *p, *q, *X[100001] = {}, *Y[100001] = {}, d[200003];
	for (i = 1; i <= N; i++) {
		d[i*2].v = y[i];
		d[i*2+1].v = x[i];
		d[i*2].next = X[x[i]];
		d[i*2+1].next = Y[y[i]];
		X[x[i]] = &(d[i*2]);
		Y[y[i]] = &(d[i*2+1]);
	}

	UF_forest F[2];
	UF_initialize(&(F[0]), 100000);
	UF_initialize(&(F[1]), 100000);
	for (i = 1; i <= 100000; i++) {
		if (X[i] == NULL || X[i]->next == NULL) continue;
		for (p = X[i]->next; p != NULL; p = p->next) if (Y[p->v] != NULL && Y[p->v]->next != NULL) break;
		if (p == NULL) continue;
		for (p = X[i]->next; p != NULL; p = p->next) {
			UF_merge(&(F[1]), X[i]->v, p->v);
			for (q = Y[p->v]->next; q != NULL; q = q->next) UF_merge(&(F[0]), i, q->v);
		}
	}
	for (i = 1; i <= 100000; i++) {
		if (Y[i] == NULL || Y[i]->next == NULL) continue;
		for (q = Y[i]->next; q != NULL; q = q->next) if (X[q->v] != NULL && X[q->v]->next != NULL) break;
		if (q == NULL) continue;
		for (q = Y[i]->next; q != NULL; q = q->next) {
			UF_merge(&(F[0]), Y[i]->v, q->v);
			for (p = Y[q->v]->next; p != NULL; p = p->next) UF_merge(&(F[1]), i, p->v);
		}
	}
	
	int r, flag[100001] = {};
	long long ans = 0;
	for (i = 1; i <= N; i++) {
		r = UF_root(&(F[0]), x[i]);
		if (flag[r] == 1) continue;
		else flag[r] = 1;
		ans += (long long)UF_size(&(F[0]), r) * UF_size(&(F[1]), y[i]);
	}
	printf("%lld\n", ans - N);
	fflush(stdout);
	return 0;
}