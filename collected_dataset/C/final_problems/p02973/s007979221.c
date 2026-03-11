#include <stdio.h>

#define MAX (1 << 17) /* 131072 */

int ki[MAX * 2 - 1];

void ki_set(int idx, int value) {
	int pos = (MAX - 1) + idx;
	ki[pos] = value;
	for (;;) {
		int c1, c2;
		pos = (pos - 1) / 2;
		c1 = ki[pos * 2 + 1];
		c2 = ki[pos * 2 + 2];
		ki[pos] = c1 >= c2 ? c1 : c2;
		if (pos == 0) break;
	}
}

int ki_get_i(int idx, int qmin, int qmax, int smin, int smax) {
	if (qmax <= smin || smax <= qmin) { /* クエリがセグメントから完全に外れている */
		return -1;
	} else if (qmin <= smin && smax <= qmax) { /* セグメントがクエリに完全に含まれる */
		return ki[idx];
	} else {
		int smid = smin + (smax - smin) / 2;
		int l = ki_get_i(idx * 2 + 1, qmin, qmax, smin, smid);
		int r = ki_get_i(idx * 2 + 2, qmin, qmax, smid, smax);
		return l >= r ? l : r;
	}
}

int ki_get(int min, int max) {
	return ki_get_i(0, min, max, 0, MAX);
}

int N;
int A[114514];

/* start以降の要素で、target以上の最初のものを探す */
int ki_mofu(int start, int target) {
	int yes = start, no = N; /* [start, yes)の要素にtarget未満のものしか無い */
	if (ki_get(start, N) < target) {
		return N;
	}
	while (yes + 1 < no) {
		int mid = yes + (no - yes) / 2;
		if (mid <= start) yes = mid;
		else if (ki_get(start, mid) < target) yes = mid; else no = mid;
	}
	return yes;
}

int main(void) {
	int i;
	int count = 0;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &A[i]) != 1) return 1;
		ki_set(i, A[i]);
	}
	while (ki_get(0, N) >= 0) {
		int current_pos, current_value;
		count++;
		current_pos = 0;
		current_value = -1;
		for (;;) {
			int next_pos = ki_mofu(current_pos, current_value + 1);
			if (next_pos >= N) break;
			current_value = A[next_pos];
			ki_set(next_pos, -1);
			current_pos = next_pos + 1;
		}
	}
	printf("%d\n", count);
	return 0;
}
