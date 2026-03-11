#include <stdio.h>
#define N_MAX 100000
void input(int *a, int l, int r) {
	for(int i = l; i <= r; ++i) {
		scanf("%i", &a[i]);
	}
	return;
}
void merge_sort(int *a, int l, int r) {
	int b[N_MAX + 1];
	for(int k = 1; k <= r - l; k *= 2) {
		int s = l, t = l + k;
		while(t <= r) {
			int so = t, to = t + k <= r + 1 ? t + k : r + 1;
			for(int i = s; i < to; ++i) {
				if(a[s] <= a[t]) {
					b[i] = a[s++];
					if(s == so) {
						for(int j = i + 1; j < to; ++j) {
							b[j] = a[t++];
						}
						break;
					}
				} else {
					b[i] = a[t++];
					if(t == to) {
						for(int j = i + 1; j < to; ++j) {
							b[j] = a[s++];
						}
						break;
					}
				}
			}
			s = t;
			t = s + k;
		}
		for(int i = l; i < s; ++i) {
			a[i] = b[i];
		}
	}
	return;
}
int greater_bound(int *a, int l, int r, int k) {
	int s = l, t = r;
	while(s <= t) {
		int m = (s + t) / 2;
		if(a[m] <= k) {
			s = m + 1;
		} else {
			t = m - 1;
		}
	}
	return s;
}
int less_bound(int *a, int l, int r, int k) {
	int s = l, t = r;
	while(s <= t) {
		int m = (s + t) / 2;
		if(a[m] < k) {
			s = m + 1;
		} else {
			t = m - 1;
		}
	}
	return t;
}
int main(int argc, char **argv) {
	int n;
	scanf("%i", &n);
	int a[N_MAX + 1];
	input(a, 1, n);
	int b[N_MAX + 1];
	input(b, 1, n);
	int c[N_MAX + 1];
	input(c, 1, n);
	merge_sort(a, 1, n);
	merge_sort(c, 1, n);
	int ans = 0;
	for(int i = 1; i <= n; ++i) {
		ans += less_bound(a, 1, n, b[i]) * (n - greater_bound(c, 1, n, b[i]) + 1);
	}
	printf("%i\n", ans);
	return 0;
}