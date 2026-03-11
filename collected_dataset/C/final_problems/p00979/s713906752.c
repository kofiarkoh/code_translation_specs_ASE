#include <stdio.h>
#include <inttypes.h>

int main(void) {
	uint64_t t;
	uint64_t current = 1;
	int depth = 0;
	uint64_t skipped = 0;
	uint64_t elapsed = 0;
	if (scanf("%" PRIu64, &t) != 1) return 1;
	if (t == 0) {
		puts("0");
		return 0;
	}
	for (;;) {
		if (skipped + current * 4 <= t) {
			depth++;
			skipped += current * 4;
			current *= 3;
			elapsed += 2;
		} else {
			break;
		}
	}
	while (skipped < t) {
		if (skipped + current <= t) {
			skipped += current;
			elapsed++;
		} else {
			current /= 3;
		}
	}
	printf("%" PRIu64 "\n", elapsed);
	return 0;
}

