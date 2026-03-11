// AOJ 0636: Foehn Phenomena
// 2017.12.14 bal4u@uu

#include <stdio.h>
#include <string.h>

long long dx[200001];
long long S, T;
char buf[40], *p;

int getint()
{
	int n = 0;
	if (*p == '-') {
		p++; while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
		p++; return -n;
	}
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	p++; return n;
}

long long calc(int i, int x)
{
	long long d;

	if (dx[i] > 0) d = dx[i]*S;
	else           d = dx[i]*T;
	dx[i] += x;
	if (dx[i] > 0) d -= dx[i]*S;
	else           d -= dx[i]*T;
	return d;
}

int main()
{
	int N, Q, i, a0, a, l, r, x;
	long long u, d, ans;
	
	fgets(p=buf, 40, stdin);
	N = getint(), Q = getint(), S = getint(), T = getint();

	fgets(buf, 10, stdin);
	u = d = a0 = 0;
	for (i = 1; i <= N; i++) {
		fgets(p=buf, 10, stdin);
		a = getint();
		a0 = a - a0;
		dx[i] = a0;
		if (a0 >= 0) u += a0;
		else         d += a0;
		a0 = a;
	}

	ans = -(d*T+u*S);
	while (Q--) {
		fgets(p=buf, 30, stdin);
		l = getint(), r = getint(), x = getint();
		ans += calc(l, x);
		if (r < N) ans += calc(r+1, -x);
		printf("%lld\n", ans);
	}
	return 0;
}