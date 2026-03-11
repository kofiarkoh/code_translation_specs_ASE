#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int key, id;
} data;

void merge_sort(data x[], int n)
{
	static data y[200001] = {};
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
	data obj[400001];
	int size;
} min_heap;

void push(data x, min_heap* h)
{
	int i = ++(h->size), j = i >> 1;
	data tmp;
	h->obj[i] = x;
	while (j > 0) {
		if (h->obj[i].key < h->obj[j].key) {
			tmp = h->obj[j];
			h->obj[j] = h->obj[i];
			h->obj[i] = tmp;
			i = j;
			j >>= 1;
		} else break;
	}
}

data pop(min_heap* h)
{
	int i = 1, j = 2;
	data output = h->obj[1], tmp;
	h->obj[1] = h->obj[(h->size)--];
	while (j <= h->size) {
		if (j < h->size && h->obj[j^1].key < h->obj[j].key) j ^= 1;
		if (h->obj[j].key < h->obj[i].key) {
			tmp = h->obj[j];
			h->obj[j] = h->obj[i];
			h->obj[i] = tmp;
			i = j;
			j <<= 1;
		} else break;
	}
	return output;
}

int main()
{
	int i, j, t, T, N, K[200001], L[200001], R[200001], tmp, flag[200001];
	long long ans;
	data d, c[200001];
	min_heap h;
	scanf("%d", &T);
	for (t = 1; t <= T; t++) {
		scanf("%d", &N);
		ans = 0;
		for (i = 1; i <= N; i++) {
			scanf("%d %d %d", &(K[i]), &(L[i]), &(R[i]));
			c[i-1].key = K[i];
			c[i-1].id = i;
			flag[i] = 0;
		}
		merge_sort(c, N);
		
		h.size = 0;
		for (i = 1, j = 0; i <= N; i++) {
			for (; j < N && c[j].key == i; j++) {
				if (L[c[j].id] > R[c[j].id]) {
					d.key = L[c[j].id] - R[c[j].id];
					d.id = c[j].id;
					push(d, &h);
				}
			}
			while (h.size > i) pop(&h);
		}
		while (h.size > 0) {
			d = pop(&h);
			flag[d.id] = 1;
			ans += L[d.id];
		}
		
		for (i = N, j = N - 1; i >= 1; i--) {
			for (; j >= 0 && c[j].key == i; j--) {
				if (R[c[j].id] > L[c[j].id]) {
					d.key = R[c[j].id] - L[c[j].id];
					d.id = c[j].id;
					push(d, &h);
				}
			}
			while (h.size > N - i) pop(&h);
		}
		while (h.size > 0) {
			d = pop(&h);
			flag[d.id] = 1;
			ans += R[d.id];
		}
		
		for (i = 1; i <= N; i++) if (flag[i] == 0) ans += (L[i] < R[i])? L[i]: R[i];
		printf("%lld\n", ans);
	}

	fflush(stdout);
	return 0;
}