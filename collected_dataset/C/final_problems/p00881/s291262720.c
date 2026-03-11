// AOJ 1302 Twenty Questions
// 2018.2.6 bal4u
 
#include <stdio.h>
#include <string.h>

int m, n;
int s[129];
char memo[20148][2048];

//#define getchar_unlocked()  getchar()
int in()
{
    int n = 0;
    int c = getchar_unlocked();
    do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
    while (c >= '0');
    return n;
}
 
int calc(int k, int yes)
{
	int i, f, x, t, t2, ans;

	if (memo[k][yes] >= 0) return memo[k][yes];
   
	f = 0;
	for (i = 0; i < n; i++) if ((s[i] & k) == yes) f++;
	if (f <= 1) return memo[k][yes] = 0;

    ans = m;
	for (i = 0; i < m; i++) {
		if ((x = k|(1<<i)) == k) continue;
		t = calc(x, yes|(1<<i)), t2 = calc(x, yes);
		if (t < t2) t = t2;
		if (ans > t+1) ans = t+1;
    }
    return memo[k][yes] = ans;
}

int main()
{
	int i, t;
	char buf[15], *p;

	while (m = in()) {
		n = in();
		for (i = 0; i < n; i++) {
			fgets(p=buf, 15, stdin);
			t = 0; while (*p > ' ') t = (t<<1) + (*p++ & 1);
			s[i] = t;
		}

		memset(memo, -1, sizeof(memo));
		printf("%d\n", calc(0, 0));
	}
	return (0);
}
