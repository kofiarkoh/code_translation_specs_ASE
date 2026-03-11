// AOJ 1389 Digits Are Not Just Characters
// 2019.2.28 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define BASE 1000000000  // >= 10^9
char s0_buf[12];
int s0[12], len;
char *bp;

// 文字列から数字部分を取り出し、バイナリに変換。文字列はグローバル変数bpで指す
int s2n()
{
	int ans = 0;
	while (isdigit(*bp)) {
		ans = 10 * ans + (*bp++ & 0xf);
	}
	return ans;
}

int main()
{
	int i, N, s, ans;
	char s_buf[12];
	
	scanf("%d", &N);
	scanf("%s", bp = s0_buf);
	len = 0;
	while (*bp) {
		if (isdigit(*bp)) s0[len] = s2n();
		else s0[len] = BASE + *bp++;
		len++;
	}
/*
	printf("len=%d:", len);
	for (i = 0; i < len; i++) printf("%d ", s0[i]); printf("\n");
*/
	while (N--) {
		scanf("%s", bp = s_buf);
		ans = 0; for (i = 0; *bp && i < len; i++) {
			if (isdigit(*bp)) s = s2n();
			else s = BASE + *bp++;
			if (s < s0[i]) { ans = -1; break; }
			if (s > s0[i]) { ans = 1; break; }
		}
		if (!ans) {
			ans = (*bp == 0 && i < len) ? -1 : 1;
		}
		putchar(ans > 0 ? '+' : '-');
		putchar('\n');
	}
	return 0;
}

