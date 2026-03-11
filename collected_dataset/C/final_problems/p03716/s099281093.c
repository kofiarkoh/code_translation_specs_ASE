#include <stdio.h>
#include <stdlib.h>

// 昇順
int compa(const void *a, const void *b) { return *(int *)a - *(int *)b; }
// 降順
int compd(const void *a, const void *b) { return *(int *)b - *(int *)a; }

int N;
int a[300000];
int left[100000];
int right[100000];

int main(void) {
	int i, left_i = 0, right_i = 0, left_ii = 0, right_ii = 0;
	long long int ans = 0;
	scanf("%d", &N);
	for (i = 0; i < N * 3; i++) {
		scanf("%d", &a[i]);
		if (i < N) {
			left[i] = a[i];
		}
		else if (i >= 2 * N) {
			right[i - 2 * N] = a[i];
		}
	}

	for (i = 0; i < N; i++) {
		ans += (a[i] - a[3 * N - 1 - i]);
	}
	int left_v, right_v;
	qsort(left, N, sizeof(int), compa);
	qsort(right, N, sizeof(int), compd);
	for (i = 0; i < N; i++) {
		left_v = a[left_i + left_ii + N] - left[left_i];
		right_v = right[right_i] - a[2 * N - 1 - right_i - right_ii];
		if (left_v < 0 && right_v < 0) {
			if (left_v - a[left_i + left_ii + N] + a[left_i + left_ii + N + 1] > right_v + a[2 * N - 1 - right_i - right_ii] - a[2 * N - 1 - right_i - right_ii - 1]) {
				left_ii++;
			}
			else {
				right_ii++;
			}
		}
		else if (left_v > right_v) {
			left_i++;
			ans += left_v;
		}
		else {
			right_i++;
			ans += right_v;
		}
	}

	printf("%lld\n", ans);

	return 0;
}