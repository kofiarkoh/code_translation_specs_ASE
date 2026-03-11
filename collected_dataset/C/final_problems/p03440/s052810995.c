#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int key, id;
} data;

void merge_sort(data x[], int n)
{
	static data y[100001] = {};
	if (n <= 1) return;
	merge_sort(&(x[0]), n/2);
	merge_sort(&(x[n/2]), (n+1)/2);
	
	int i, p, q;
	for (i = 0, p = 0, q = n/2; i < n; i++) {
		if (p >= n/2) y[i] = x[q++];
		else if (q >= n) y[i] = x[p++];
		else y[i] = (x[p].key < x[q].key)? x[p++]: x[q++];
	}
	for (i = 0; i < n; i++) x[i] = y[i];
}

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
	int i, u, w, N, M;
	scanf("%d %d", &N, &M);
	
	if (N - 1 - M > N / 2) {
		printf("Impossible\n");
		fflush(stdout);
		return 0;
	}
	
	data a[100001];
	for (i = 0; i < N; i++) {
		scanf("%d", &(a[i].key));
		a[i].id = i + 1;
	}
	merge_sort(a, N);
	
	UF_forest F;
	UF_initialize(&F, N);
	for (i = 1; i <= M; i++) {
		scanf("%d %d", &u, &w);
		UF_merge(&F, u + 1, w + 1);
	}
	
	int k, r, count[100001] = {}, comp = 0;
	long long ans = 0;
	for (i = 0, k = 0; k < (N - 1 - M) * 2; i++) {
		r = UF_root(&F, a[i].id);
		if (count[r] == 0) {
			comp++;
			count[r]++;
			ans += a[i].key;
			k++;
		} else if (count[r] < N - 1 - M && (N - 1 - M) * 2 - k > N - M - comp) {
			count[r]++;
			ans += a[i].key;
			k++;
		}
	}
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}