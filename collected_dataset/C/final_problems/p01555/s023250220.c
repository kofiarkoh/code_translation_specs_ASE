// Aizu 2441: FizzBuzz
// 2017.9.26 bal4u@uu

#include <stdio.h>
#include <string.h>

char ans[500];

long long calc(long long a)
{
	int j;
	long long l, r, p, ans;
	
	ans = (a/3 + a/5) << 2;
	for (j = 0, p = 1; p <= a; p*=10) {
		j++;
		r = p*10; if (r > a) r = a;
		l = p - 1;
		ans += ((r-r/3-r/5+r/15) - (l-l/3-l/5+l/15))*j;
	}
	return ans;
}

int main()
{
	long long s, i, k, l, r, m;
	char *p;

	scanf("%lld", &s);
	l = 0, r = 200000000000000000LL;
	while (l + 1 < r) {
		m = (l + r) >> 1;
		if (calc(m) >= s) r = m;
		else              l = m;
	}
	for (p = ans, k = l+1, i = 1; i < 20; i++, k++) {
		if      (k % 15 == 0) memcpy(p, "FizzBuss", 8), p += 8;
		else if (k %  3 == 0) memcpy(p, "Fizz", 4), p += 4;
		else if (k %  5 == 0) memcpy(p, "Buzz", 4), p += 4;
		else {
			sprintf(p, "%lld", k); while (*p) p++;
		}
	}
	*p = 0;
	k = s - calc(l) - 1;
	ans[k+20] = 0;
	puts(ans+k);
	return 0;
}