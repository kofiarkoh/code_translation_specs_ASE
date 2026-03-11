#include <stdio.h>
#include <inttypes.h>

#define KI_MAX (1 << 18) /* 262144 */

#define INF 1010101010

struct ki_t {
	int data[KI_MAX * 2 - 1];
	int all_data[KI_MAX * 2 - 1];
};

int ayaneru(int a, int b) {
	return a <= b ? a : b;
}

void ki_init(struct ki_t* ki) {
	int i;
	for (i = 0; i < KI_MAX * 2 - 1; i++) {
		ki->data[i] = ki->all_data[i] = INF;
	}
}

void ki_upd_i(struct ki_t* ki, int idx, int qs, int qe, int ss, int se, int value) {
	if (qe <= ss || se <= qs) { /* はずれ */
		/* nop */
	} else if (qs <= ss && se <= qe) { /* セグメントがクエリに完全に含まれる */
		ki->all_data[idx] = ayaneru(ki->all_data[idx], value);
	} else {
		int sm = ss + (se - ss) / 2;
		int li = idx * 2 + 1, ri = idx * 2 + 2;
		ki->all_data[li] = ayaneru(ki->all_data[li], ki->all_data[idx]);
		ki->all_data[ri] = ayaneru(ki->all_data[ri], ki->all_data[idx]);
		ki->all_data[idx] = INF;
		ki_upd_i(ki, li, qs, qe, ss, sm, value);
		ki_upd_i(ki, ri, qs, qe, sm, se, value);
		ki->data[idx] = ayaneru(ayaneru(ki->all_data[li], ki->data[li]), ayaneru(ki->all_data[ri], ki->data[ri]));
	}
}

void ki_upd(struct ki_t* ki, int qs, int qe, int value) {
	if (qs < qe) ki_upd_i(ki, 0, qs, qe, 0, KI_MAX, value);
}

int ki_get_i(const struct ki_t* ki, int idx, int qs, int qe, int ss, int se) {
	if (qe <= ss || se <= qs) { /* はずれ */
		return INF;
	} else if (qs <= ss && se <= qe) { /* セグメントがクエリに完全に含まれる */
		return ayaneru(ki->data[idx], ki->all_data[idx]);
	} else {
		int sm = ss + (se - ss) / 2;
		int l = ki_get_i(ki, idx * 2 + 1, qs, qe, ss, sm);
		int r = ki_get_i(ki, idx * 2 + 2, qs, qe, sm, se);
		return ayaneru(ayaneru(l, r), ki->all_data[idx]);
	}
}

int ki_get(const struct ki_t* ki, int qs, int qe) {
	return qs < qe ? ki_get_i(ki, 0, qs, qe, 0, KI_MAX) : INF;
}

int N, Q;
int type[212345], x[212345];

struct ki_t tate_ni_limit, yoko_ni_limit;

int main(void) {
	int i;
	int64_t minus_count = 0;
	if (scanf("%d%d", &N, &Q) != 2) return 1;
	for (i = 0; i < Q; i++) {
		if (scanf("%d%d", &type[i], &x[i]) != 2) return 1;
	}
	ki_init(&tate_ni_limit);
	ki_init(&yoko_ni_limit);
	ki_upd(&tate_ni_limit, 1, N + 1, N);
	ki_upd(&yoko_ni_limit, 1, N + 1, N);
	for (i = 0; i < Q; i++) {
		if (type[i] == 1) { /* 縦 */
			int limit = ki_get(&yoko_ni_limit, x[i], x[i] + 1);
			minus_count += (limit - 1) - 1;
			ki_upd(&tate_ni_limit, 1, limit, x[i]);
		} else if (type[i] == 2) { /* 横 */
			int limit = ki_get(&tate_ni_limit, x[i], x[i] + 1);
			minus_count += (limit - 1) - 1;
			ki_upd(&yoko_ni_limit, 1, limit, x[i]);
		}
	}
	printf("%" PRId64 "\n", (int64_t)(N - 2) * (N - 2) - minus_count);
	return 0;
}
