#include <stdio.h>

const int sup = 1 << 30;

typedef struct {
	int id;
	long long key;
} data;

typedef struct {
	data obj[100001];
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
	int i, N, L[100001], R[100001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d %d", &(L[i]), &(R[i]));

	int min = R[1], argmin = 1;
	for (i = 2; i <= N; i++) {
		if (R[i] < min) {
			min = R[i];
			argmin = i;
		}
	}

	data d;
	max_heap h;
	h.size = 0;
	for (i = 1; i <= N; i++) {
		if (i == argmin) continue;
		d.key = (long long)L[i] * sup + R[i];
		push(d, &h);
	}
	
	int ans = 0, l[2] = {L[argmin], h.obj[0].key / sup}, r[2] = {R[argmin], h.obj[0].key % sup};
	while (h.size > 0 && h.obj[0].key / sup > r[0]) {
		d = pop(&h);
		l[1] = (l[1] < d.key / sup)? d.key / sup: l[1];
		r[1] = (r[1] > d.key % sup)? d.key % sup: r[1];
	}
	while (h.size > 0) {
		l[0] = h.obj[0].key / sup;
		if (r[0] - l[0] + r[1] - l[1] > ans) ans = r[0] - l[0] + r[1] - l[1];
		while (h.size > 0 && h.obj[0].key / sup == l[0]) {
			d = pop(&h);
			r[1] = (r[1] > d.key % sup)? d.key % sup: r[1];
		}
	}
	l[0] = L[argmin];
	if (r[0] - l[0] + r[1] - l[1] > ans) ans = r[0] - l[0] + r[1] - l[1];
	
	int max = R[argmin] - L[argmin], argmax = argmin;
	for (i = 1; i <= N; i++) {
		if (R[i] - L[i] > max) {
			max = R[i] - L[i];
			argmax = i;
		}
	}
	if (argmax != argmin) {
		for (i = 1, max = 0; i <= N; i++) {
			if (i == argmax) continue;
			if (L[i] > max) max = L[i];
		}
		if (R[argmax] - L[argmax] + ((max <= min)? min - max: 0) > ans) ans = R[argmax] - L[argmax] + ((max <= min)? min - max: 0);
	}
	
	printf("%d\n", ans + 2);
	fflush(stdout);
	return 0;
}