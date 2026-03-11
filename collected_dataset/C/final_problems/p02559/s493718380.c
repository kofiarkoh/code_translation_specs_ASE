#include <stdio.h>
#include <inttypes.h>

#define KI_MAX 512345

uint64_t ki[KI_MAX];

void ki_add(int idx, uint64_t value) {
	if (idx <= 0) return;
	for (; idx < KI_MAX; idx += idx & -idx) {
		ki[idx] += value;
	}
}

uint64_t ki_sum(int idx) {
	uint64_t sum = 0;
	for (; idx > 0; idx -= idx & -idx) {
		sum += ki[idx];
	}
	return sum;
}

int main(void) {
	int N, Q;
	int i;
	if (scanf("%d%d", &N, &Q) != 2) return 1;
	for (i = 0; i < N; i++) {
		int a;
		if (scanf("%d", &a) != 1) return 1;
		ki_add(i + 1, a);
	}
	for (i = 0; i < Q; i++) {
		int q[3];
		if (scanf("%d%d%d", &q[0], &q[1], &q[2]) != 3) return 1;
		if (q[0] == 0) {
			ki_add(q[1] + 1, q[2]);
		} else if (q[0] == 1) {
			printf("%" PRIu64 "\n", ki_sum(q[2]) - ki_sum(q[1]));
		}
	}
	return 0;
}
