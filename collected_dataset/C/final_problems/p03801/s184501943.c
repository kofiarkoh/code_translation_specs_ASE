#include <stdio.h>

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

int main()
{
	int i, N;
	data d[100001];
	scanf("%d", &N);
	for (i = 0; i < N; i++) {
		scanf("%d", &(d[i].key));
		d[i].id = i + 1;
	}
	merge_sort(d, N);
	
	int j, k, min[100001], count[100001], v[100001];
	for (i = N - 1, k = 0; i >= 0; i--) {
		v[k] = d[i].key;
		for (j = i - 1; j >= 0 && d[j].key == v[k]; j--);
		count[k] = i - j;
		for (min[k] = d[i].id; i > j; i--) if (d[i].id < min[k]) min[k] = d[i].id;
		i++;
		k++;
	}
	min[k] = 1;
	count[k] = 0;
	v[k] = 0;
	
	long long ans[100001] = {};
	for (i = 0; i < k; i++) {
		ans[min[i]] += (long long)(v[i] - v[i+1]) * count[i];
		min[i+1] = (min[i] < min[i+1])? min[i]: min[i+1];
		count[i+1] += count[i];
	}
	
	for (i = 1; i <= N; i++) printf("%lld\n", ans[i]);
	fflush(stdout);
	return 0;
}
