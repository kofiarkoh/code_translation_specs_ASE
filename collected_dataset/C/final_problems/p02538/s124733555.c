#include <stdio.h>
#include <inttypes.h>

#define MOD_BY  998244353

int add(int a, int b) {
	return a + b - MOD_BY * (a + b >= MOD_BY);
}

int mul(int a, int b) {
	return (int)(((int64_t)a * b) % MOD_BY);
}

#define MAX (1 << 18) /* 262144 */

int mult_11[MAX], mult_10[MAX];

void get_mult(void) {
	int i;
	mult_10[0] = 1;
	for (i = 1; i < MAX; i++) mult_10[i] = mul(mult_10[i - 1], 10);
	mult_11[0] = 0;
	for (i = 1; i < MAX; i++) mult_11[i] = add(mul(mult_11[i - 1], 10), 1);
}

int ki[MAX * 2 - 1], ki_all[MAX * 2 - 1];

int get_value(int idx, int width) {
	if (ki_all[idx] >= 0) {
		return mul(ki_all[idx], mult_11[width]);
	} else {
		return ki[idx];
	}
}

void ki_init(void) {
	int i;
	for (i = 0; i < MAX * 2 - 1; i++) {
		ki[i] = 0;
		ki_all[i] = -1;
	}
}

void ki_set_i(int idx, int qs, int qe, int ss, int se, int value) {
	if (qe <= ss || se <= qs) { /* 完全に外れている */
		/* nop */
	} else if (qs <= ss && se <= qe) { /* セグメントがクエリに完全に含まれる */
		ki_all[idx] = value;
	} else {
		int sm = ss + (se - ss) / 2;
		int lidx = idx * 2 + 1, ridx = idx * 2 + 2;
		if (ki_all[idx] >= 0) {
			ki_all[lidx] = ki_all[ridx] = ki_all[idx];
			ki_all[idx] = -1;
		}
		ki_set_i(lidx, qs, qe, ss, sm, value);
		ki_set_i(ridx, qs, qe, sm, se, value);
		ki[idx] = add(mul(get_value(lidx, sm - ss), mult_10[se - sm]), get_value(ridx, se - sm));
	}
}

void ki_set(int qs, int qe, int value) {
	if (qs < qe) ki_set_i(0, qs, qe, 0, MAX, value);
}

int ki_get_i(int idx, int qs, int qe, int ss, int se) {
	if (qe <= ss || se <= qs) { /* 完全に外れている */
		return 0;
	} else if (qs <= ss && se <= qe) { /* セグメントがクエリに完全に含まれる */
		return get_value(idx, se - ss);
	} else if (ki_all[idx] >= 0) { /* 値が設定されている */
		int left = qs <= ss ? ss : qs;
		int right = se <= qe ? se : qe;
		return mul(ki_all[idx], mult_11[right - left]);
	} else {
		int rpos = qe <= se ? qe : se;
		int sm = ss + (se - ss) / 2;
		int left = ki_get_i(idx * 2 + 1, qs, qe, ss, sm);
		int right = ki_get_i(idx * 2 + 2, qs, qe, sm, se);
		if (rpos <= sm) {
			return left;
		} else {
			return add(mul(left, mult_10[rpos - sm]), right);
		}
	}
}

int ki_get(int qs, int qe) {
	return qs < qe ? ki_get_i(0, qs, qe, 0, MAX) : 0;
}

int main(void) {
	int N, Q;
	int i;
	if (scanf("%d%d", &N, &Q) != 2) return 1;
	get_mult();
	ki_init();
	ki_set(1, 1 + N, 1);
	for (i = 0; i < Q; i++) {
		int L, R, D;
		if (scanf("%d%d%d", &L, &R, &D) != 3) return 1;
		ki_set(L, R + 1, D);
		printf("%d\n", ki_get(1, 1 + N));
	}
	return 0;
}
