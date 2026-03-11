#include <stdio.h>
#include <inttypes.h>

int main(void) {
	int N;
	int64_t kake = 1, tasi = 0;
	int i;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		int q, x;
		if (scanf("%d%d", &q, &x) != 2) return 1;
		switch (q) {
			case 1:
				kake *= x;
				tasi *= x;
				break;
			case 2:
				tasi += x;
				break;
			case 3:
				tasi -= x;
				break;
		}
	}
	printf("%" PRId64 " %" PRId64 "\n", -tasi, kake);
	return 0;
}

