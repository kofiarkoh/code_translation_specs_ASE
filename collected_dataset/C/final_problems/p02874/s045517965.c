#include <stdio.h>

typedef struct {
	int key, id;
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

	int ans, max = L[1], min = R[1], argmax = 1, argmin = 1;
	for (i = 2; i <= N; i++) {
		if (L[i] > max) {
			max = L[i];
			argmax = i;
		}
		if (R[i] < min) {
			min = R[i];
			argmin = i;
		}
	}
	for (i = 1, max = 0; i <= N; i++) {
		if (i == argmax || i == argmin) continue;
		if (R[i] - L[i] > max) max = R[i] - L[i];
	}
	if (argmax == argmin) {
		printf("%d\n", max + R[argmin] - L[argmax] + 2);
		fflush(stdout);
		return 0;
	} else if (R[argmin] >= L[argmax]) ans = max + R[argmin] - L[argmax] + 2;
	else ans = max + 1;

	data d;
	max_heap h;
	h.size = 0;
	for (i = 1; i <= N; i++) {
		if (i == argmax || i == argmin) continue;
		d.key = L[i];
		d.id = R[i];
		push(d, &h);
	}
	
	int l[2] = {L[argmin], L[argmax]}, r[2] = {R[argmin], R[argmax]}, tmp[2][2];
	while (h.size > 0) {
		d = pop(&h);
		tmp[0][0] = (l[0] < d.key)? d.key: l[0];
		tmp[0][1] = (r[0] > d.id)? d.id: r[0];
		tmp[1][0] = (l[1] < d.key)? d.key: l[1];
		tmp[1][1] = (r[1] > d.id)? d.id: r[1];
		if (tmp[0][1] < tmp[0][0]) tmp[0][1] = tmp[0][0] - 1;
		if (tmp[1][1] < tmp[1][0]) tmp[1][1] = tmp[1][0] - 1;
		if (r[0] - l[0] - (tmp[0][1] - tmp[0][0]) <= r[1] - l[1] - (tmp[1][1] - tmp[1][0])) {
			l[0] = tmp[0][0];
			r[0] = tmp[0][1];
		} else {
			l[1] = tmp[1][0];
			r[1] = tmp[1][1];
		}
	}
	
	if (r[0] - l[0] + r[1] - l[1] + 2 > ans) printf("%d\n", r[0] - l[0] + r[1] - l[1] + 2);
	else printf("%d\n", ans);
	fflush(stdout);
	return 0;
}