#include <stdio.h>

int main()
{
	int N, Q;
	char S[500001];
	scanf("%d", &N);
	scanf("%s", S);
	scanf("%d", &Q);
	
	int i, j, count[501][26] = {};
	for (i = 0; i < N / 1000; i++) {
		for (j = 0; j < 1000; j++) count[i][S[i*1000+j] - 'a']++;
	}
	for (j = 0; j < N % 1000; j++) count[i][S[i*1000+j] - 'a']++;

	int l, r, q, type, flag[26], ans;
	char c;
	for (q = 0; q < Q; q++) {
		scanf("%d", &type);
		if (type == 1) {
			scanf("%d %c", &i, &c);
			i--;
			count[i/1000][S[i] - 'a']--;
			count[i/1000][c - 'a']++;
			S[i] = c;
		} else {
			scanf("%d %d", &l, &r);
			l--;
			r--;
			for (j = 0; j < 26; j++) flag[j] = 0;
			for (i = (l + 999) / 1000; i < (r + 1) / 1000; i++) {
				for (j = 0; j < 26; j++) if (count[i][j] > 0) flag[j] = 1;
			}
			for (j = l; j <= r && j % 1000 != 0; j++) flag[S[j] - 'a'] = 1;
			for (j = r; j >= l && (j + 1) % 1000 != 0; j--) flag[S[j] - 'a'] = 1;
			for (j = 0, ans = 0; j < 26; j++) ans += flag[j];
			printf("%d\n", ans);
		}
	}
	
	fflush(stdout);
	return 0;
}