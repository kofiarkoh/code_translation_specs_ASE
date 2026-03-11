#include <stdio.h>
#include <string.h>
int main(void) {
	int n, i, j, k;
	char cp;
	scanf("%d", &n);
	char S[n][100];
	int count[26] = {};
	int ans[26] = {};
	int flag = 0;
	for (i = 0; i < n; i++) scanf("%s", S[i]);

	for (i = 0; i < n; i++) {
		for (k = 0; k < 26; k++)count[k] = 0;
		for (j = 0; j < strlen(S[i]); j++) {
			//printf("%d",strlen(S[i]));
			count[S[i][j] - 'a']++;
		}
		for (k = 0; k < 26; k++) {
			if (flag == 0) {
				ans[k] = count[k];
			}
			else if (ans[k] > count[k]) {
				ans[k] = count[k];
			}
			//printf("%c:%d\n", 'a' + k, count[k]);
		}
		flag = 1;
	}

	for (i = 0; i < 26; i++) {
		for (; ans[i] != 0; ans[i]--) printf("%c", 'a' + i);
	}
	printf("\n");



	return 0;
}