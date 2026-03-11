// AOJ 2130: Billion Million Thousand
// 2017.11.10 bal4u@uu

#include <stdio.h>
#include <string.h>

typedef struct { char s[102]; int w, p; } T;
T tbl[101];
char goal[202]; int len;
int dp1[202], dp2[2000];

int main()  
{  
	int n, i, j, k;
	int cno = 0;

	while (scanf("%d", &n) && n > 0) {
		for (i = 0; i < n; i++) {
			scanf("%s%d", &tbl[i].s, &tbl[i].p);
			tbl[i].w = strlen(tbl[i].s);
		}
		scanf("%s", goal); len = strlen(goal);

		memset(dp1, -1, sizeof(dp1)); dp1[0] = 0;
		memset(dp2, 0x33, sizeof(dp2)); dp2[0] = 0;
		for (i = 0; i <= len; i++) for (j = 0; j < n; j++) {
			k = i - tbl[j].w;
			if (k < 0 || dp1[k] < 0) continue;
			if (!memcmp(goal+k, tbl[j].s, tbl[j].w)) {
				if (dp1[i] < dp1[k] + tbl[j].p) dp1[i] = dp1[k] + tbl[j].p;
			}
		}
		for (k = dp1[len], i = 0; i < n; i++) for (j = 0; j <= k-tbl[i].p; j++) {
			int t = dp2[j] + tbl[i].w;
			if (t < dp2[j+tbl[i].p]) dp2[j+tbl[i].p] = t;
		}
		printf("Case %d: %d\n", ++cno, dp2[k]);
    }  
    return 0;  
}  