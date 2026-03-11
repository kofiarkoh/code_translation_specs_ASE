// AOJ 2539: Counting 1's
// 2017.12.1 bal4u@uu

#include <stdio.h>
#include <stdlib.h>

#define INF 1000000000000000000LL
int n;

long long k[65];

long long calc(long long a, int x)
{
	long long mask, t, ans;

    mask = 1LL << x;
	a++;
//    t = a/mask, ans = (t >> 1)*mask;
    t = a >> x, ans = (t >> 1)*mask;
    if (t & 1) ans += a & (mask-1);
    return ans;
}

int check(long long a, long long b)
{
	int i;

    for (i = n-1; i >= 0; i--)
        if (calc(b, i) - calc(a-1, i) != k[i]) return 0;
    return 1;
}

long long solve(long long w)
{
	int i;
    long long a = 0, b = -1, c = -1;

    for (i = n-1; i >= 0; i--) {
        if (k[i] > w) return -1;
	    if (k[i] == w) a |= 1LL << i;
        else if (k[i]) {
            b = a | (1LL<<i);
            c = b + k[i]-1;
            a = c - w + 1;
            break;
        }
    }
    if (c == -1) c = a;
    if (a == 0 || c > INF) return -1;
    if (a + w - 1 == c && check(a, c)) return a;
    return -1;
}

char buf[50], *p;
long long getlong()
{
	long long n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int main()
{
	int i;
	long long a, b, w, ans;

	while (fgets(p=buf, 5, stdin) && *p != '0') {
		n = atoi(buf);
		for (i = 0; i < n; i++) {
			fgets(p=buf, 30, stdin);
			k[i] = getlong();
		}

        a = -1;
        for (w = 2*k[0]-1; w <= 2*k[0]+1; w++) {
			if (w <0) continue;
            ans = solve(w);
            if (ans != -1) {
				if (a != -1) { puts("Many"); goto done; }
                a = ans; b = ans + w - 1;
            }
        }
        if (a == -1) puts("None");
        else printf("%lld %lld\n", a, b);
done:   ;
	}
    return 0;
}