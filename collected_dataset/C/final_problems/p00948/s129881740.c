#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int x, y;
} aamu_t;

int hikaku(const void* x, const void* y) {
	int a = ((const aamu_t*)x)->x;
	int b = ((const aamu_t*)y)->x;
	if (a > b) return 1;
	if (a < b) return -1;
	return 0;
}

int n, m;
aamu_t aamu[100000];

typedef struct {
	int ue, sita;
} hani_t;

hani_t hani[200000];

int main(void) {
	int i;
	if (scanf("%d%d", &n, &m) != 2) return 1;
	for (i = 0; i < m; i++) {
		if (scanf("%d%d", &aamu[i].x, &aamu[i].y) != 2) return 1;
	}
	qsort(aamu, m, sizeof(*aamu), hikaku);
	for (i = 0; i < n; i++) {
		hani[i].ue = hani[i].sita = i;
	}
	for (i = 0; i < m; i++) {
		int y = aamu[i].y;
		int ue = hani[y - 1].ue < hani[y].ue ? hani[y - 1].ue : hani[y].ue;
		int sita = hani[y].sita > hani[y - 1].sita ? hani[y].sita : hani[y - 1].sita;
		hani[y - 1].ue = hani[y].ue = ue;
		hani[y - 1].sita = hani[y].sita = sita;
	}
	for (i = 0; i < n; i++) {
		printf("%d", hani[i].sita - hani[i].ue + 1);
		putchar(i + 1 < n ? ' ' : '\n');
	}
	return 0;
}