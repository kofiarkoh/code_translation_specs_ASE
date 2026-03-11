#include <stdio.h>
#include <inttypes.h>

int l, k;

uint64_t kugyu[128];

uint64_t mizuhasu(int left) {
	uint64_t hocchann = 1;
	if (left < 0) return 0;
	if (kugyu[left]) return ~kugyu[left];

	hocchann += mizuhasu(left - 1 - 1);
	hocchann += mizuhasu(left - 1 - k);

	return ~(kugyu[left] = ~hocchann);
}

int main(void) {
	if (scanf("%d%d", &l, &k) != 2) return 1;
	printf("%" PRIu64 "\n", mizuhasu(l - 1) + mizuhasu(l - k));
	return 0;
}

