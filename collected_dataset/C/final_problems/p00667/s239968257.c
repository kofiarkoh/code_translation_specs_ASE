// AOJ 1082: 11224111122411
// 2017.11.1 bal4u@uu

#include <stdio.h>

#define N 100000
#define M 100000007

int a[N+2] = {0,1,2,4,8,16};
int b[N+2] = {0,1,2,4};

char buf[N+2], *p, *q;

int main()
{
	int i, j;
	long long ans;

	for (i = 6; i <= N; i++) {
		for (a[i] = 1, j = 1; j < 6; j++) a[i] = (a[i] + a[i-j]) % M;
	}
	for (i = 4; i <= N; i++) {
		for (b[i] = 1, j = 1; j < 4; j++) b[i] = (b[i] + b[i-j]) % M;
	}
	while (fgets(buf, sizeof(buf), stdin) && *buf != '#') {
		ans = 1;
		p = buf; while (*p >= '0') {
			q = p; while (*q >= '0' && *q == *p) q++;
			if (*p == '8' || *p == '0') ans = (ans * b[q-p]) % M;
			else						ans = (ans * a[q-p]) % M;
			p = q;
		}
		printf("%lld\n", ans);
	}
	return 0;
}