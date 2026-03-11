#include <stdio.h>
#include <limits.h>
#define MAX_N 100

int sum(int *data, int start, int end) {
	int i, s = 0;

	for (i = start; i < end; i++) {
		s += data[i];
	}
	return s;
}

int find(int n, int data[MAX_N][MAX_N]) {
	int temp[5100], row, index, num_cell, end, i, k, s, maximum;

	maximum = INT_MIN;
	for (row = 0; row < n; row++) {
		index = 0;
		num_cell = 1;

		for (end = n; end >= 0; end--) {
			for (i = 0; i < end; i++) {
				k = index + i;

				s = sum(data[row], i, i+num_cell);
				if (temp[k] < 0) {
					temp[k] = s;
				} else {
					temp[k] += s;
				}
				if (maximum < temp[k]) {
					maximum = temp[k];
				}
			}
			index += end;
			num_cell++;
		}
	}
	return maximum;
}

int main(void) {
	int n, i, j;
	int a[MAX_N][MAX_N];

	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		for (j = 0; j < n; j++) {
			scanf("%d", &a[i][j]);
		}
	}
	printf("%d\n", find(n, a));
	return 0;
}