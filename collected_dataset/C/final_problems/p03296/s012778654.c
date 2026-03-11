#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stdint.h>
#include <string.h>
#include <wchar.h>

#define MIN(a, b) (a < b ? a : b)
#define MAX(a, b) (a > b ? a : b)
#define ABSS(a, b) ((a) > (b) ? (a) - (b) : (b) - (a))

uint64_t a[100];

static int compare_int(const void *a, const void *b) {
	return (int)(*(int64_t *)a - *(int64_t *)b);
}

int main(void) {
	uint64_t N;
	uint32_t count = 0;

	scanf("%"PRIu64"\n", &N);
	for (uint64_t idx = 0; idx < N; idx++) {
		scanf(idx != N - 1 ? "%"PRIu64" " : "%"PRIu64, &(a[idx]));
		if (idx != 0) {
			if (a[idx] == a[idx - 1]) {
				count++;
				a[idx] = 100 + count;
			}
		}
	}

	printf("%"PRIu32, count);

	return 0;
}
