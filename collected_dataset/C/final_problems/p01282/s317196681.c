// AOJ 2164 Revenge of the Round Table
// 2018.3.22 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

int gcd(int a, int b)
{
	int r;
	while (b != 0) r = a % b, a = b, b = r;
	return a;
}

#define M 1000003

int n, k;
int inv[1002];
int dpa[1002][1002], dpb[1002][1002], dp[1002];

void calc()
{
	int i, j, t, sa, sb;

    memset(dpa, 0, sizeof(dpa));
    memset(dpb, 0, sizeof(dpb));
    memset(dp, 0, sizeof(dp));

    sa = 1, sb = 0;
    dpa[1][1] = 1;
    dpb[1][1] = 0;
 
    for (i = 2; i <= n; i++) {
        dpa[i][1] = sb, dpb[i][1] = sa;
		t = sa, sa = sb, sb = t;
        for (j = 2; j <= k; j++) {
            dpa[i][j] = dpa[i-1][j-1];
            sa = sa + dpa[i][j]; if (sa >= M) sa -= M;
            dpb[i][j] = dpb[i-1][j-1];
            sb = sb + dpb[i][j]; if (sb >= M) sb -= M;
        }
    }

    for (i = 1; i <= n; i++) for (j = 1; j <= k; j++) {
		dp[i] += dpb[i][j]; if (dp[i] >= M) dp[i] -= M;
	}
 
    for (i = 1; i <= n; i++) for (j = 1; j < k; j++) {
		dpb[i][j+1] += dpb[i][j]; 
		if (dpb[i][j+1] >= M) dpb[i][j+1] -= M;
    }
 
    for (i = 1; i <= n; i++) {
		t = i; if (t > k) t = k; 
        for (j = 1; j <= t; j++) {
            dp[i] += dpb[i-j][k-j]; if (dp[i] >= M) dp[i] -= M;
        }
    }
}

int main()
{
	int s, i;
	long long ans;

    inv[1] = 1; for (i = 2; i <= 1000; i++)
		inv[i] = M - (long long)(M/i) * inv[M%i] % M;

	while (n = in()) {
		k = in();

		s = 0;
	    if (k >= n) k = n-1, s = 2;
		calc();
		ans = 0; for (i = 0; i < n; i++) ans = (ans + (dp[gcd(i, n)] << 1)) % M;
        printf("%d\n", (int)((s + ans * inv[n]) % M));
	}
	return 0;
}
