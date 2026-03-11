// AOJ 2595: Cookie Counter
// 2017.11.30 bal4u@uu

#include <stdio.h>
#include <string.h>

#define M 1000000007
#define MAX 2002
long long a[MAX];
long long b[MAX];

long long egcd(long long a, long long b, long long *x, long long *y)
{
	long long d;

	if (b == 0) { *x = 1; *y = 0; return a; }
	d = egcd(b, a % b, y, x);
	*y -= a / b * (*x);
	return d;
}

long long invMod(long long a)
{
  long long x, y;
  if (egcd(a, M, &x, &y) == 1) return (x + M) % M;
  return 0;
}

int main()
{
	int n, x, i, ii;
	long long d, ans, t;

    while (scanf("%d%lld%d", &n, &d, &x) && n > 0) {
		a[0] = b[0] = 1;
		for (i = 0; i <= n; i++) {
			t = invMod(i + 1);
			a[i+1] = (a[i] * ((d+i) % M) % M * t) % M;
			b[i+1] = (b[i] * ((d-i) % M) % M * t) % M;
		}
		ans = 0;
		for (ii = n/x, i = 0; i <= ii; i++) {
			t = a[n-x*i] * b[i] % M;
			if (i & 1) ans = (ans - t) % M;
			else	   ans = (ans + t) % M;
			if (ans < 0) ans += M;
		}	
        printf("%lld\n", ans);
	}
    return 0;
}