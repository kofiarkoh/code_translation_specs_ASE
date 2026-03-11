#include <stdio.h>
#include <inttypes.h>
 
int N, K;
int H[512];
 
/* [最初, 2番目) */
int64_t sakugenTable[512][512];
 
int64_t memo[512][512];
 
int64_t search(int pos, int stock) {
	int64_t answer;
	int i;
	if (pos > N) return 0;
	if (memo[pos][stock]) return ~memo[pos][stock];
 
	/* 使わない */
	answer = search(pos + 1, stock);
	/* 使う */
	for(i = 1; i <= stock && pos + i <= N + 1; i++) {
		int64_t candidate = search(pos + i + 1, stock - i) + sakugenTable[pos][pos + i];
		if (candidate > answer) answer = candidate;
	}
 
	return ~(memo[pos][stock] = ~answer);
}
 
int main(void) {
	int i, j;
	int64_t all_cost;
	if (scanf("%d%d", &N, &K) != 2) return 1;
	for (i = 1; i <= N; i++) {
		if (scanf("%d", &H[i]) != 1) return 1;
	}
	H[0] = H[N + 1] = 0;
	for (i = 1; i <= N; i++) {
		int64_t sum = 0;
		for (j = i; j <= N + 1; j++) {
			if (H[j] < H[j - 1]) sum += H[j - 1] - H[j];
			sakugenTable[i][j] = sum + H[j];
		}
	}
	all_cost = sakugenTable[1][N + 1];
	for (i = 1; i <= N; i++) {
		for (j = i; j <= N + 1; j++) {
			int64_t after_cost = (H[i - 1] >= H[j]) ? H[i - 1] : H[j];
			sakugenTable[i][j] -= after_cost;
		}
	}
	printf("%" PRId64 "\n", all_cost - search(1, K));
	return 0;
}