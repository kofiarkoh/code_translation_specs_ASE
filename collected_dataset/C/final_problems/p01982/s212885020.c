// AOJ 2882: Generalized Leap Years
// 2018.7.11 bal4u

#include <stdio.h>
#include <string.h>

int year[4001];
int A[50];

int main()
{
	int i, j, n, l, r, k, v, ans;
	
	while (scanf("%d%d%d", &n, &l, &r) && n) {
		for (i = 0; i < n; i++) scanf("%d", A+i);
		memset(year, 0, sizeof(year));
		v = 2 - (n&1);	// v = 1 if n is odd
		j = n;
		while (j--) {
			for (i = l/A[j]*A[j]; i <= r; i += A[j]) year[i] = v;
			v = 3-v;
		}
		ans = k = 0;
		for (i = l; i <= r; i++) {
			if (!year[i]) k++;
			else if (year[i] == 1) ans++;
		}
		if (!(n & 1)) ans += k;
		printf("%d\n", ans);
	}
	return 0;
}
