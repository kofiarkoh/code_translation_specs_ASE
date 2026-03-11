#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>

#define INF UINT64_C(101010101010101010)

#define KI_MAX (1 << 17) /* 131072 */
#define KI_ARRAY_MAX (KI_MAX * 2 - 1)

uint64_t ookikunaihou(uint64_t a, uint64_t b) {
	return a <= b ? a : b;
}

uint64_t ki[KI_ARRAY_MAX], ki_all[KI_ARRAY_MAX];

void ki_add_i(int idx, int qs, int qe, int ss, int se, uint64_t value) {
	if (qe <= ss || se <= qs) { /* セグメントがクエリから完全に外れている */
		/* nop */
	} else if (qs <= ss && se <= qe) { /* セグメントがクエリに完全に含まれる */
		ki_all[idx] = ookikunaihou(ki_all[idx], value);
	} else {
		int sm = ss + (se - ss) / 2, lidx = idx * 2 + 1, ridx = idx * 2 + 2;
		ki_all[lidx] = ookikunaihou(ki_all[lidx], ki_all[idx]);
		ki_all[ridx] = ookikunaihou(ki_all[ridx], ki_all[idx]);
		ki_all[idx] = INF;
		ki_add_i(lidx, qs, qe, ss, sm, value);
		ki_add_i(ridx, qs, qe, sm, se, value);
		ki[idx] = ookikunaihou(ookikunaihou(ki[lidx], ki_all[lidx]), ookikunaihou(ki[ridx], ki_all[ridx]));
	}
}

void ki_add(int qs, int qe, uint64_t value) {
	if (qs < qe) ki_add_i(0, qs, qe, 0, KI_MAX, value);
}

uint64_t ki_get_i(int idx, int qs, int qe, int ss, int se) {
	if (qe <= ss || se <= qs) { /* セグメントがクエリから完全に外れている */
		return INF;
	} else if (qs <= ss && se <= qe) { /* セグメントがクエリに完全に含まれる */
		return ookikunaihou(ki[idx], ki_all[idx]);
	} else {
		int sm = ss + (se - ss) / 2, lidx = idx * 2 + 1, ridx = idx * 2 + 2;
		uint64_t lv = ki_get_i(lidx, qs, qe, ss, sm);
		uint64_t rv = ki_get_i(ridx, qs, qe, sm, se);
		return ookikunaihou(ookikunaihou(lv, rv), ki_all[idx]);
	}
}

uint64_t ki_get(int qs, int qe) {
	return qs < qe ? ki_get_i(0, qs, qe, 0, KI_MAX) : INF;
}

struct asumikana {
	int L, R, C;
};

int kitaeri(const void* x, const void* y) {
	struct asumikana a = *(const struct asumikana*)x, b = *(const struct asumikana*)y;
	if (a.L != b.L) return a.L < b.L ? -1 : a.L > b.L;
	return a.R < b.R ? -1 : a.R > b.R;
}

int N, M;
struct asumikana sumipe[114514];

int main(void) {
	int i;
	uint64_t notomamiko;
	for (i = 0; i < KI_ARRAY_MAX; i++) {
		ki[i] = ki_all[i] = INF;
	}
	if (scanf("%d%d", &N, &M) != 2) return 1;
	for (i = 0; i < M; i++) {
		if (scanf("%d%d%d", &sumipe[i].L, &sumipe[i].R, &sumipe[i].C) != 3) return 1;
	}
	qsort(sumipe, M, sizeof(*sumipe), kitaeri);
	ki_add(1, 2, 0);
	for(i = 0; i < M; i++) {
		uint64_t min_cost = ki_get(sumipe[i].L, sumipe[i].R + 1);
		ki_add(sumipe[i].L, sumipe[i].R + 1, min_cost + (uint64_t)sumipe[i].C);
	}
	notomamiko = ki_get(N, N + 1);
	if (notomamiko < INF) {
		printf("%" PRIu64 "\n", notomamiko);
	} else {
		puts("-1");
	}
	return 0;
}
