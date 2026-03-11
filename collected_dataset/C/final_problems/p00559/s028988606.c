#include <stdio.h>
#include <inttypes.h>

#define MAX (1 << 18) /* 262144 */
#define INF INT64_C(99999999999999999)

int64_t ki[MAX * 2 - 1], ki_all[MAX * 2 - 1];

void ki_add_i(int idx, int qs, int qe, int qdelta, int ss, int se) {
	if (qe <= ss || se <= qs) { /* query ga segument kara kanzen ni hazurete iru */
		/* NOP */
	} else if (qs <= ss && se <= qe) { /* segment ga query ni kanzen ni fukumareru */
		ki_all[idx] += qdelta;
	} else {
		int sm = ss + (se - ss) / 2;
		int lidx = idx * 2 + 1, ridx = idx * 2 + 2;
		int64_t l, r;
		ki_all[lidx] += ki_all[idx];
		ki_all[ridx] += ki_all[idx];
		ki_all[idx] = 0;
		ki_add_i(lidx, qs, qe, qdelta, ss, sm);
		ki_add_i(ridx, qs, qe, qdelta, sm ,se);
		l = ki[lidx] + ki_all[lidx];
		r = ki[ridx] + ki_all[ridx];
		ki[idx] = l >= r ? l : r;
	}
}

void ki_add(int s, int e, int delta) {
	ki_add_i(0, s, e, delta, 0, MAX);
}

int64_t ki_get_i(int idx, int qs, int qe, int ss, int se) {
	if (qe <= ss || se <= qs) { /* query ga segument kara kanzen ni hazurete iru */
		return -INF;
	} else if (qs <= ss && se <= qe) { /* segment ga query ni kanzen ni fukumareru */
		return ki[idx] + ki_all[idx];
	} else {
		int sm = ss + (se - ss) / 2;
		int lidx = idx * 2 + 1, ridx = idx * 2 + 2;
		int64_t l, r;
		l = ki_get_i(lidx, qs, qe, ss, sm);
		r = ki_get_i(ridx, qs, qe, sm ,se);
		return (l >= r ? l : r) + ki_all[idx];
	}
}

int64_t ki_get(int idx) {
	return ki_get_i(0, idx, idx + 1, 0, MAX);
}

int N, Q, S, T;
int A[222222];
int L[222222], R[222222], X[222222];

int64_t getDelta(int64_t from, int64_t to) {
	if (from >= to) {
		return (int64_t)T * (from - to);
	} else {
		return -(int64_t)S * (to - from);
	}
}

int main(void) {
	int i;
	int64_t kaze = 0;
	if (scanf("%d%d%d%d", &N, &Q, &S, &T) != 4) return 1;
	for (i = 0; i <= N; i++) {
		if (scanf("%d", &A[i]) != 1) return 1;
	}
	for (i = 0; i < Q; i++) {
		if (scanf("%d%d%d", &L[i], &R[i], &X[i]) != 3) return 1;
	}
	ki_add(0, 1, A[0]);
	for (i = 1; i <= N; i++) {
		ki_add(i, i + 1, A[i]);
		kaze += getDelta(A[i - 1], A[i]);
	}
	for (i = 0; i < Q; i++) {
		int64_t prev = ki_get(L[i] - 1);
		int64_t next = ki_get(R[i] + 1);
		int64_t asumi = ki_get(L[i]);
		int64_t iguti = ki_get(R[i]);
		ki_add(L[i], R[i] + 1, X[i]);
		kaze -= getDelta(prev, asumi);
		kaze += getDelta(prev, ki_get(L[i]));
		if (R[i] < N) {
			kaze -= getDelta(iguti, next);
			kaze += getDelta(ki_get(R[i]), next);
		}
		printf("%" PRId64 "\n", kaze);
	}
	return 0;
}

