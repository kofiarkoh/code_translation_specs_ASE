#include <stdio.h>
#include <string.h>
#include <inttypes.h>

int nyan(uint64_t x) {
	x = ((x & UINT64_C(0xAAAAAAAAAAAAAAAA)) >>  1) + (x & UINT64_C(0x5555555555555555));
	x = ((x & UINT64_C(0xCCCCCCCCCCCCCCCC)) >>  2) + (x & UINT64_C(0x3333333333333333));
	x = ((x & UINT64_C(0xF0F0F0F0F0F0F0F0)) >>  4) + (x & UINT64_C(0x0F0F0F0F0F0F0F0F));
	x = ((x & UINT64_C(0xFF00FF00FF00FF00)) >>  8) + (x & UINT64_C(0x00FF00FF00FF00FF));
	x = ((x & UINT64_C(0xFFFF0000FFFF0000)) >> 16) + (x & UINT64_C(0x0000FFFF0000FFFF));
	x = ((x & UINT64_C(0xFFFFFFFF00000000)) >> 32) + (x & UINT64_C(0x00000000FFFFFFFF));
	return (int)x;
}

#define MAX 500001
#define CHUNK 708
#define BIT 64

#define CHUNK_NUM ((MAX + CHUNK - 1) / CHUNK)
#define SET_NUM ((MAX + BIT - 1) / BIT)

uint64_t sets[CHUNK_NUM][SET_NUM];

int N, Q;
int c[512345];
int l[512345];
int r[512345];

uint64_t data[SET_NUM];

int main(void) {
	int i;
	if (scanf("%d%d", &N, &Q) != 2) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &c[i]) != 1) return 1;
	}
	for (i = 0; i < Q; i++) {
		if (scanf("%d%d", &l[i], &r[i]) != 2) return 1;
		l[i]--;
		r[i]--;
	}

	for (i = 0; i < N; i++) {
		sets[i / CHUNK][c[i] / BIT] |= UINT64_C(1) << (c[i] % BIT);
	}

	for (i = 0; i < Q; i++) {
		int start_set = l[i] / CHUNK;
		int end_set = r[i] / CHUNK;
		int j;
		int answer = 0;
		memset(data, 0, sizeof(data));
		if (start_set == end_set) {
			for (j = l[i]; j <= r[i]; j++) {
				data[c[j] / BIT] |= UINT64_C(1) << (c[j] % BIT);
			}
		} else {
			for (j = l[i]; j < CHUNK * start_set; j++) {
				data[c[j] / BIT] |= UINT64_C(1) << (c[j] % BIT);
			}
			for (j = CHUNK * (end_set - 1); j <= r[i]; j++) {
				data[c[j] / BIT] |= UINT64_C(1) << (c[j] % BIT);
			}
			for (j = start_set + 1; j < end_set; j++) {
				int k;
				for (k = 0; k < SET_NUM; k++) data[k] |= sets[j][k];
			}
		}
		for (j = 0; j < SET_NUM; j++) answer += nyan(data[j]);
		printf("%d\n", answer);
	}

	return 0;
}
