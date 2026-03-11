// AOJ 1077: The Great Summer Contest
// 2017.10.14

#include <stdio.h>
#include <string.h>

int n[3];
int k[4];

int main()
{
	int i, j, a, f, ans;

	while (1) {
		memset(n, 0, sizeof(n)); memset(k, 0, sizeof(k));
		for (f = 0, j = 0; j < 2; j++) for (i = 0; i < 3; i++)
			scanf("%d", &a), n[i] += a, f |= a;
		if (!f) break;

		for (ans = 0, i = 0; i < 3; i++) {
			if (n[i] > 3) {	ans += n[i] / 3, n[i] %= 3; }
			k[n[i]]++;
		}
		if (k[0] > 0) ans += k[3];
		else if (k[1] > 0) {
			if (k[3] > 0) ans += k[3]; else ans++;
		} else if (k[2] > 0) ans += 2;
		else ans += k[3];
		printf("%d\n", ans);
	}
	return 0;
}