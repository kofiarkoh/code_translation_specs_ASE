#include <stdio.h>

typedef struct {
	long long key;
	int id;
} data;

void merge_sort(data x[], int n)
{
	static data y[200001] = {};
	if (n == 1) return;
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
	int i, N;
	long long D, A, X[200001], H[200001];
	scanf("%d %lld %lld", &N, &D, &A);
	for (i = 1; i <= N; i++) scanf("%lld %lld", &(X[i]), &(H[i]));
	
	data d[200001];
	for (i = 0; i < N; i++) {
		d[i].key = X[i+1];
		d[i].id = i + 1;
	}
	merge_sort(d, N);
	
	int j, k, l, r, b[200001][2] = {}, head, tail, tmp;
	long long ans = 0;
	for (i = 0, head = 0, tail = 0, tmp = 0; i < N; i++) {
		for (; head < tail && b[head][0] < i; head++) tmp -= b[head][1];
		if (H[d[i].id] <= tmp) continue;

		ans += (H[d[i].id] - tmp + A - 1) / A;
		for (l = i, r = N - 1; l < r; ) {
			if (d[(l+r+1)/2].key > d[i].key + D * 2) r = (l + r - 1) / 2;
			else l = (l + r + 1) / 2;
		}
		b[tail][0] = l;
		b[tail][1] = ((H[d[i].id] - tmp + A - 1) / A) * A;
		tmp += b[tail++][1];
	}
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}