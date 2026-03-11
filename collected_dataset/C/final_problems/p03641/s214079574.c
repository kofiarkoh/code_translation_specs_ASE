#include <stdio.h>

#define SIZE 400
#define NUM 501

int main()
{
	int i, N, p[200001];
	scanf("%d", &N);
	for (i = 0; i < N; i++) scanf("%d", &(p[i]));
	
	int j, k = (N + SIZE - 1) / SIZE, min[2][NUM], argmin[2][NUM];
	for (i = 0; i < k; i++) {
		min[0][i] = p[i*SIZE];
		min[1][i] = p[i*SIZE+1];
		argmin[0][i] = i * SIZE;
		argmin[1][i] = i * SIZE + 1;
		for (j = i * SIZE + 2; j < N && j % SIZE != 0; j++) {
			if (p[j] < min[j%2][i]) {
				min[j%2][i] = p[j];
				argmin[j%2][i] = j;
			}
		}
	}
	
	int l, r, n, tmp, parity[NUM] = {}, flag[NUM] = {}, stop[200001] = {}, ans[200001];
	for (n = 0; n < N; n += 2) {
		for (i = 0, ans[n] = N + 1; i < k; i++) {
			if (min[0][i] < ans[n]) {
				ans[n] = min[0][i];
				l = argmin[0][i];
			}
		}
		for (j = l + 1, ans[n+1] = N + 1; j < N && j / SIZE == l / SIZE && stop[j-1] + stop[j] == 0; j += 2) {
			if (p[j] < ans[n+1]) {
				ans[n+1] = p[j];
				r = j;
			}
		}
		if (j < N && stop[j-1] + stop[j] == 0) {
			for (i = j / SIZE; i < k && flag[i] == 0; i++) {
				if (min[1][i] < ans[n+1]) {
					ans[n+1] = min[1][i];
					r = argmin[1][i];
				}
			}
			if (i < k) {
				for (j = i * SIZE + (l + 1) % 2; j < N && stop[j-1] + stop[j] == 0; j += 2) {
					if (p[j] < ans[n+1]) {
						ans[n+1] = p[j];
						r = j;
					}
				}
			}
		}
		
		stop[l] = 1;
		stop[r] = 1;
		i = l / SIZE;
		flag[i] = 1;
		for (j = i * SIZE, tmp = parity[i], min[0][i] = N + 1, min[1][i] = N + 1; j < N && j < (i + 1) * SIZE; j++) {
			if (stop[j] == 1) continue;
			if (p[j] < min[tmp][i]) {
				min[tmp][i] = p[j];
				argmin[tmp][i] = j;
			}
			tmp ^= 1;
		}
		if (r / SIZE > i) {
			for (i++; i < r / SIZE; i++) {
				parity[i] ^= 1;
				min[0][i] += min[1][i];
				min[1][i] = min[0][i] - min[1][i];
				min[0][i] -= min[1][i];
				argmin[0][i] += argmin[1][i];
				argmin[1][i] = argmin[0][i] - argmin[1][i];
				argmin[0][i] -= argmin[1][i];
			}
			flag[i] = 1;
			for (j = i * SIZE, parity[i] ^= 1, tmp = parity[i], min[0][i] = N + 1, min[1][i] = N + 1; j < N && j < (i + 1) * SIZE; j++) {
				if (stop[j] == 1) continue;
				if (p[j] < min[tmp][i]) {
					min[tmp][i] = p[j];
					argmin[tmp][i] = j;
				}
				tmp ^= 1;
			}
		}
	}
	
	for (i = 0; i < N - 1; i++) printf("%d ", ans[i]);
	printf("%d\n", ans[N-1]);
	fflush(stdout);
	return 0;
}