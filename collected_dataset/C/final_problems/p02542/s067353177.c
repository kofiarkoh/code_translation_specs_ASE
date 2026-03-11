#include <stdio.h>

int main()
{
	int i, N, M;
	char S[52][52] = {};
	scanf("%d %d", &N, &M);
	for (i = 1; i <= N; i++) scanf("%s", &(S[i][1]));

	int j, k, l, ans = 0, dist[2501], q[2501], head, tail;
	for (i = N; i >= 1; i--) {
		for (j = M; j >= 1; j--) {
			if (S[i][j] != 'o') continue;
			for (k = 1; k <= N * M; k++) dist[k] = 10000;
			dist[(i-1)*M+j] = 0;
			q[0] = (i - 1) * M + j;
			for (head = 0, tail = 1; head < tail; head++) {
				k = (q[head] - 1) / M + 1;
				l = (q[head] - 1) % M + 1;
				if (S[k+1][l] == '.') {
					dist[k*M+l] = dist[q[head]] + 1;
					q[tail++] = k * M + l;
				}
				if (S[k][l+1] == '.') {
					dist[(k-1)*M+l+1] = dist[q[head]] + 1;
					q[tail++] = (k - 1) * M + l + 1;
				}
			}
			ans += dist[q[head-1]];
			k = (q[head-1] - 1) / M + 1;
			l = (q[head-1] - 1) % M + 1;
			S[i][j] = '.';
			S[k][l] = 'o';
		}
	}
	printf("%d\n", ans);
	fflush(stdout);
	return 0;
}