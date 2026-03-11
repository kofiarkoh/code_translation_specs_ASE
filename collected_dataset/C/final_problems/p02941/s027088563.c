#include <stdio.h>

typedef struct {
	int key, id;
} data;

typedef struct {
	data obj[200001];
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

int main()
{
	int i, N, A[200001], B[200001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", &(A[i]));
	for (i = 1; i <= N; i++) scanf("%d", &(B[i]));

	data d;
	max_heap h;
	h.size = 0;
	for (i = 1; i <= N; i++) {
		if (A[i] > B[i]) break;
		else if (A[i] == B[i]) continue;
		d.key = B[i];
		d.id = i;
		push(d, &h);
	}
	if (i <= N) {
		printf("-1\n");
		fflush(stdout);
		return 0;
	}
	
	int j, k, n = 0;
	while (h.size > 0) {
		d = pop(&h);
		j = d.id;
		i = (j + N - 2) % N + 1;
		k = j % N + 1;
		if (B[j] <= B[i] + B[k] || B[j] % (B[i] + B[k]) < A[j]) break;
		n += B[j] / (B[i] + B[k]);
		B[j] %= B[i] + B[k];
		if (B[j] > A[j]) {
			d.key = B[j];
			push(d, &h);
		}
	}

	for (i = 1; i <= N; i++) if (A[i] != B[i]) break;
	if (i > N) printf("%d\n", n);
	else printf("-1\n");
	fflush(stdout);
	return 0;
}