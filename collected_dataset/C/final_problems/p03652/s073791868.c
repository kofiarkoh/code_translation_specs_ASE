#include <stdio.h>

typedef struct {
	int key, id;
} data;

void merge_sort(data x[], int n)
{
	static data y[302] = {};
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

int main()
{
	int i, j, N, M, A[301][301];
	scanf("%d %d", &N, &M);
	for (i = 1; i <= N; i++) {
		for (j = 1; j <= M; j++) scanf("%d", &(A[i][j]));
	}
	
	int k, m, min = 1000, flag[301] = {}, p[302];
	data d[302];
	for (i = 1; i <= M; i++) flag[i] = 1;
	for (m = M; m >= 1; ) {
		for (i = 1, k = 0; i <= M; i++) {
			if (flag[i] == 1) {
				d[k].key = 0;
				d[k].id = i;
				p[i] = k++;
			}
		}
		
		for (i = 1; i <= N; i++) {
			for (j = 1; j <= M; j++) {
				if (flag[A[i][j]] == 1) {
					d[p[A[i][j]]].key++;
					break;
				}
			}
		}
		merge_sort(d, k);
		if (d[k-1].key < min) min = d[k-1].key;
		for (flag[d[--k].id] = 0, m--; k > 0 && d[k].key == d[k-1].key; flag[d[--k].id] = 0, m--);
	}
	
	printf("%d\n", min);
	fflush(stdout);
	return 0;
}