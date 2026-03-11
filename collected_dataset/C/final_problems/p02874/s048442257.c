#include <stdio.h>
#include <inttypes.h>

static unsigned int x=62526377;
static unsigned int y=5463654;
static unsigned int z=87602543;
static unsigned int w=13597657;

void seed(unsigned int a,unsigned int b,unsigned int c,unsigned int d) {
	if((a|b|c|d)==0)return;
	x=a;y=b;z=c;w=d;
}

unsigned int randint(void) {
	unsigned int t;
	t=(x^(x<<11));
	x=y;y=z;z=w;
	w=(w^(w>>19))^(t^(t>>8));
	return w;
}

#define MAX 200000

int32_t N;
struct lr_t {
	int32_t L, R;
} LR[MAX];

int32_t solve(int32_t seed1, int32_t seed2) {
	int32_t i;
	int32_t al, ar, bl, br;
	/* 損が少ない方に入れる貪欲 */
	al = LR[seed1].L; ar = LR[seed1].R;
	bl = LR[seed2].L; br = LR[seed2].R;
	for (i = 0; i < N; i++) {
		int32_t al2 = al, ar2 = ar, bl2 = bl, br2 = br;
		int32_t psa = ar - al, psb = br - bl, asa, asb;
		if (i == seed1 || i == seed2) continue;
		if (al2 < LR[i].L) al2 = LR[i].L;
		if (LR[i].R < ar2) ar2 = LR[i].R;
		if (bl2 < LR[i].L) bl2 = LR[i].L;
		if (LR[i].R < br2) br2 = LR[i].R;
		if (ar2 < al2) ar2 = al2;
		if (br2 < bl2) br2 = bl2;
		asa = ar2 - al2;
		asb = br2 - bl2;
		if (((psa - asa) < (psb - asb)) ^ (randint() % 100 == 0)) {
			al = al2; ar = ar2;
		} else {
			bl = bl2; br = br2;
		}
	}

	return (ar - al) + (br - bl);
}

int main(void) {
	int32_t i;
	int32_t ai = 0, bi = 0, asmax = -1, bsmax = -1;
	int32_t best = 0, candidate;
	int32_t r, s;
	if (scanf("%" SCNd32, &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		if  (scanf("%" SCNd32 "%" SCNd32, &LR[i].L, &LR[i].R) != 2) return 1;
		LR[i].R++; /* [L, R) に変換 */
	}

	for (r = 0; r < 20; r++) {
		/* やけくそシャッフル */
		for (s = N - 1; s > 0; s--) {
			int32_t idx = randint() % (s + 1);
			struct lr_t temp = LR[s];
			LR[s] = LR[idx];
			LR[idx] = temp;
		}

		/* 一番長いのを選ぶ */
		asmax = -1;
		for (i = 0; i < N; i++) {
			int32_t as = LR[i].R - LR[i].L;
			if (as > asmax) { ai = i; asmax = as; }
		}
		/* それと一番共通部分が少ないのを選ぶ */
		bsmax = MAX;
		for (i = 0; i < N; i++) {
			int32_t l = LR[ai].L < LR[i].L ? LR[i].L : LR[ai].L;
			int32_t r = LR[i].R < LR[ai].R ? LR[i].R : LR[ai].R;
			int32_t bs;
			if (i == ai) continue;
			bs = l < r ? r - l : 0;
			if (bs < bsmax) { bi = i; bsmax = bs; }
		}
		candidate = solve(ai, bi);
		if (best < candidate) best = candidate;

		/* Lが一番左のを選ぶ */
		asmax = MAX;
		for (i = 0; i < N; i++) {
			int32_t as = LR[i].L;
			if (as < asmax) { ai = i; asmax = as; }
		}
		/* Rが一番右のを選ぶ */
		bsmax = -1;
		for (i = 0; i < N; i++) {
			int32_t bs;
			if (i == ai) continue;
			bs = LR[i].R;
			if (bs > bsmax) { bi = i; bsmax = bs; }
		}
		candidate = solve(ai, bi);
		if (best < candidate) best = candidate;

		/* Rが一番左のを選ぶ */
		asmax = MAX;
		for (i = 0; i < N; i++) {
			int32_t as = LR[i].R;
			if (as < asmax) { ai = i; asmax = as; }
		}
		/* Lが一番右のを選ぶ */
		bsmax = -1;
		for (i = 0; i < N; i++) {
			int32_t bs;
			if (i == ai) continue;
			bs = LR[i].L;
			if (bs > bsmax) { bi = i; bsmax = bs; }
		}
		candidate = solve(ai, bi);
		if (best < candidate) best = candidate;

		/* 一番短いのを選ぶ */
		asmax = MAX;
		for (i = 0; i < N; i++) {
			int32_t as = LR[i].R - LR[i].L;
			if (as < asmax) { ai = i; asmax = as; }
		}
		/* 二番目に短いのを選ぶ */
		bsmax = MAX;
		for (i = 0; i < N; i++) {
			int32_t bs;
			if (i == ai) continue;
			bs = LR[i].R - LR[i].L;
			if (bs > bsmax) { bi = i; bsmax = bs; }
		}
		candidate = solve(ai, bi);
		if (best < candidate) best = candidate;

		/* やけくそ乱択 */
		for (i = 0; i < 100; i++) {
			ai = randint() % N;
			bi = randint() % (N - 1);
			if (ai <= bi) bi++;
			candidate = solve(ai, bi);
			if (best < candidate) best = candidate;
		}
	}

	printf("%" PRId32 "\n", best);

	return 0;
}

/*
各コンテスト最初の1問を除いて、問題をコンテストに入れたとき
* 必ず解ける人は減るか変わらない (増えない)
* 必ず範囲の左端は変わらないか右に動く
* 必ず範囲の右端は変わらないか左に動く

予想：一番長いのと一番短いのは別のコンテストにするべき
これは偽
3
1 10
9 15
11 100

-> 92

入力例1
12345678
   ****
****
    ****
 ****

もし、共通部分が無い問題の組があるとき
同じコンテストに入れる -> 楽しさ0になる -> もう1方は残りから一番長い1問を取ってほかを全部ぶちこむ
違うコンテストに入れる -> ？

1. 各コンテスト最初の1問を決める
2. 残りを適当に割り振る

「1本とそれ以外」を超えるパフォーマンスを出せるか？

*/

/*

4
1 10
11 101
2 100
12 13

-> 99

**********
 ******************************************
          **********************************
           *


*/
