// AOJ 2335: 10-Year-Old Dynamic Programming
// 2017.11.27 bal4u@uu
// 2017.12.2

#include <stdio.h>

#define M 1000000007

int fact[2020001];

int egcd(int a, int b, int *x, int *y)
{
	int d;

	if (b == 0) { *x = 1; *y = 0; return a; }
	d = egcd(b, a % b, y, x);
	*y -= a / b * (*x);
	return d;
}

int inverse(int a)
{
    int x, y;
	egcd(a, M, &x, &y);
	if (x < 0) x += M;
	return x;
}

int comb(int n, int k)
{
//    if (n < 0 || k < 0 || n < k) return 0;
	if (k == 0) return 1;
    return (long long)fact[n] * inverse(((long long)fact[k]*fact[n-k]) % M) % M;
}

int main()
{
	int n, m, k, i, j, kk, ii, jj, r, c, ans;

	for (fact[0] = 1, i = 1; i < 2020001; i++) fact[i] = ((long long)fact[i-1]*i) % M;

	scanf("%d%d%d", &n, &m, &k);

    ans = 0; kk = n + m + (k << 1);
    for (j = k, i = 0; i <= k; i++, j--) {
        ii = n + (i << 1), jj = m + (j << 1);
        r = comb(ii, i) - comb(ii, i-1); if (r < 0) r += M;
        c = comb(jj, j) - comb(jj, j-1); if (c < 0) c += M;
        ans = (ans + ((long long)r*c) % M * (long long)comb(kk, ii)) % M;
    }
    printf("%d\n", ans);
}