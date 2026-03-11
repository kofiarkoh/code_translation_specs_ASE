#include <stdio.h>
#include <stdlib.h>

/* {1,2,3} -> ... -> {3,2,1} */
int next_permutation(int arr[],int n) {
	int target;
	int i;
	int temp;
	for (i = n - 2; i >= 0; i--) {
		if (arr[i] < arr[i + 1]) break;
	}
	if (i < 0) return 0;
	target = i;

	for (i = 0; target + i + 1 < n - i - 1; i++) {
		temp = arr[target + 1 + i];
		arr[target + 1 + i] = arr[n - 1 - i];
		arr[n - i - 1] = temp;
	}

	for (i = target + 1; i < n; i++) {
		if (arr[i] > arr[target]) break;
	}
	temp = arr[i];
	arr[i] = arr[target];
	arr[target] = temp;
	return 1;
}

int N;
char S[114514];

int count[10][10];
int cost[10][10];

int main(void) {
	int i, j;
	int bestScore = -1;
	int bestHaiti[9] = {0};
	int haiti[9] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
	if (scanf("%d", &N) != 1) return 1;
	if (scanf("%114513s", S) != 1) return 1;
	for (i = 1; i < N; i++) {
		count[S[i - 1] - '0'][S[i] - '0']++;
	}
	do {
		int score;
		for (i = 0; i < 9; i++) {
			int x1 = i % 3;
			int y1 = i / 3;
			for (j = i + 1; j < 9; j++) {
				int x2 = j % 3;
				int y2 = j / 3;
				cost[haiti[i]][haiti[j]] = cost[haiti[j]][haiti[i]] = abs(x1 - x2) + abs(y1 - y2);
			}
			cost[haiti[i]][haiti[i]] = 0;
		}
		score = 0;
		for (i = 1; i <= 9; i++) {
			for (j = 1; j <= 9; j++) {
				score += count[i][j] * cost[i][j];
			}
		}
		if (bestScore < 0 || score < bestScore) {
			bestScore = score;
			for (i = 0; i < 9; i++) bestHaiti[i] = haiti[i];
		}
	} while (next_permutation(haiti, 9));
	for (i = 0; i < 3; i++) {
		printf("%d%d%d\n", bestHaiti[i * 3], bestHaiti[i * 3 + 1], bestHaiti[i * 3 + 2]);
	}
	return 0;
}

