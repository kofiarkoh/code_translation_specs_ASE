#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int key, id;
} data;

typedef struct {
	data *obj;
	int size;
} max_heap;

void push(data x, max_heap* h)
{
	int i = h->size;
	data tmp;
	h->obj[(h->size)++] = x;
	while (1) {
		if (i == 0) break;
		else if (h->obj[i].key > h->obj[(i-1)/2].key) {
			tmp = h->obj[(i-1)/2];
			h->obj[(i-1)/2] = h->obj[i];
			h->obj[i] = tmp;
			i = (i - 1) / 2;
		} else break;
	}
}

data pop(max_heap* h)
{
	int i = 0, j;
	data output = h->obj[0], tmp;
	h->obj[0] = h->obj[--(h->size)];
	while (1) {
		if (i * 2 + 1 >= h->size) break;
		else if (i * 2 + 2 >= h->size) j = i * 2 + 1;
		else if (h->obj[i*2+1].key > h->obj[i*2+2].key) j = i * 2 + 1;
		else j = i * 2 + 2;
		if (h->obj[j].key > h->obj[i].key) {
			tmp = h->obj[j];
			h->obj[j] = h->obj[i];
			h->obj[i] = tmp;
			i = j;
		} else break;
	}
	return output;
}

typedef struct {
	int *par, *size, *height, *flag;
} UF_forest;

void UF_initialize(UF_forest *F, int n)
{
	int i;
	F->par = (int*)malloc(sizeof(int) * (n + 1));
	F->size = (int*)malloc(sizeof(int) * (n + 1));
	F->height = (int*)malloc(sizeof(int) * (n + 1));
	F->flag = (int*)malloc(sizeof(int) * (n + 1));
	for (i = 1; i <= n; i++) {
		F->par[i] = i;
		F->size[i] = 1;
		F->height[i] = 1;
		F->flag[i] = 0;
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
		F->flag[u] |= F->flag[v];
	} else {
		F->par[u] = v;
		F->size[v] += F->size[u];
		F->flag[v] |= F->flag[u];
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
	int i, N, H, W, A[100001], R[100001], C[100001];
	scanf("%d %d %d", &N, &H, &W);
	for (i = 1; i <= N; i++) scanf("%d %d %d", &(R[i]), &(C[i]), &(A[i]));
	
	data d;
	max_heap h;
	h.size = 0;
	h.obj = (data*)malloc(sizeof(data) * (N + 1));
	for (i = 1; i <= N; i++) {
		d.key = A[i];
		d.id = i;
		push(d, &h);
	}
	
	int r[2];
	long long ans = 0;
	UF_forest F;
	UF_initialize(&F, H + W);
	while (h.size > 0) {
		d = pop(&h);
		r[0] = UF_root(&F, R[d.id]);
		r[1] = UF_root(&F, C[d.id] + H);
		if (r[0] != r[1] && F.flag[r[0]] + F.flag[r[1]] <= 1) {
			ans += d.key;
			UF_merge(&F, r[0], r[1]);
		} else if (F.flag[r[0]] == 0) {
			ans += d.key;
			F.flag[r[0]] = 1;
		}
	}
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}