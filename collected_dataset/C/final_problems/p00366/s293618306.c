#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>

int cmp(const void* x, const void* y) {
	int a = *(const int*)x, b = *(const int*)y;
	return a < b ? -1 : a > b;
}

int N;
int t[114514];

int64_t count[114514];
int64_t cost[114514];

int soinsuCount;
int soinsu[32][2];

int yakusuCount;
int yakusu[114514];

void getYakusu(int idx, int current) {
	if (idx >= soinsuCount) {
		yakusu[yakusuCount++] = current;
	} else {
		int mofu = current;
		int i;
		for (i = 0; i <= soinsu[idx][1]; i++) {
			getYakusu(idx + 1, mofu);
			mofu *= soinsu[idx][0];
		}
	}
}

int main(void) {
	int i;
	int makkusu = 0;
	int64_t answer = 0;
	int kugyu;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &t[i]) != 1) return 1;
		count[t[i]]++;
		if (t[i] > makkusu) makkusu = t[i];
	}
	for (i = 1; i <= makkusu; i++) {
		count[i] += count[i - 1];
		cost[i] = cost[i - 1] + count[i - 1];
	}
	kugyu = makkusu;
	soinsuCount = 0;
	for (i = 2; i * i <= kugyu; i++) {
		if (kugyu % i == 0) {
			int c = 0;
			while (kugyu % i == 0) {
				kugyu /= i;
				c++;
			}
			soinsu[soinsuCount][0] = i;
			soinsu[soinsuCount][1] = c;
			soinsuCount++;
		}
	}
	if (kugyu > 1) {
		soinsu[soinsuCount][0] = kugyu;
		soinsu[soinsuCount][1] = 1;
		soinsuCount++;
	}
	yakusuCount = 0;
	getYakusu(0, 1);
	qsort(yakusu, yakusuCount, sizeof(*yakusu), cmp);
	answer = cost[yakusu[0]];
	for (i = 1; i < yakusuCount; i++) {
		answer += cost[yakusu[i]] - (count[yakusu[i - 1]] * (yakusu[i] - yakusu[i - 1])) - cost[yakusu[i - 1]];
	}
	printf("%" PRId64 "\n", answer);
	return 0;
}

/*
sukunakutomo itiban nagai yatu yori mizikaku dekinai
*/

