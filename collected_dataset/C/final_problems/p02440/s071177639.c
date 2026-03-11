#include <stdio.h>
#include <limits.h>

#define MAX_NUM 1024

int min_data[MAX_NUM * 2 - 1];
int max_data[MAX_NUM * 2 - 1];

int get_min(int a, int b) {
	return a <= b ? a : b;
}

int get_max(int a, int b) {
	return a >= b ? a : b;
}

int segtree(const int array[], int (*merge)(int, int), int inf_value,
int idx, int idx_left, int idx_right, int q_left, int q_right) {
	if (idx_right <= q_left || q_right <= idx_left) return inf_value;
	else if (q_left <= idx_left && idx_right <= q_right) return array[idx];
	else {
		int center = (idx_right - idx_left) / 2 + idx_left;
		int lv = segtree(array, merge, inf_value, idx * 2 + 1, idx_left, center, q_left, q_right);
		int rv = segtree(array, merge, inf_value, idx * 2 + 2, center, idx_right, q_left, q_right);
		return merge(lv, rv);
	}
}

int get_min_segtree(int begin, int end) {
	return segtree(min_data, get_min, INT_MAX, 0, 0, MAX_NUM, begin, end);
}

int get_max_segtree(int begin, int end) {
	return segtree(max_data, get_max, INT_MIN, 0, 0, MAX_NUM, begin, end);
}

int main(void) {
	int n, q, i;
	if (scanf("%d", &n) != 1) return 1;
	for (i = 0; i < n; i++) {
		if (scanf("%d", &min_data[MAX_NUM - 1 + i]) != 1) return 1;
		max_data[MAX_NUM - 1 + i] = min_data[MAX_NUM - 1 + i];
	}
	for (i = MAX_NUM - 1 - 1; i >= 0; i--) {
		min_data[i] = get_min(min_data[i * 2 + 1], min_data[i * 2 + 2]);
		max_data[i] = get_max(max_data[i * 2 + 1], max_data[i * 2 + 2]);
	}
	if (scanf("%d", &q) != 1) return 1;
	for (i = 0; i < q; i++) {
		int com, b, e;
		if (scanf("%d%d%d", &com, &b, &e) != 3) return 1;
		printf("%d\n", (com == 0 ? get_min_segtree : get_max_segtree)(b, e));
	}
	return 0;
}

