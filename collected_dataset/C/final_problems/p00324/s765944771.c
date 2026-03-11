// AOJ 0329: Bilateral Trade
// 2017.8.26 bal4u@uu
// 2017.11.14

#include <stdio.h>

#define MAX 200000

#define HASHSIZ 370003LL
#define BASE 200000000000001LL        // -200,000 x 10^9
typedef struct { int i; long long s; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash+HASHSIZ;

int insert(int i, long long s)
{
    HASH *p;
    s += BASE;
    p = hash + s % HASHSIZ;
    while (p->s) {
        if (p->s == s) return p->i;
        if (++p == hashend) p = hash;
    }
    p->s = s, p->i = i;
    return -1;
}

char buf[20], *p;
int getint()
{
	int n = 0;
	if (*p == '-') {
		p++; while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
		return -n;
	}
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int main()
{
//  int N, d;
	int N;
    int i, k, max;
    long long s0, s;

//	memset(hash, 0, sizeof(hash));
//  scanf("%d", &N);
	fgets(p=buf, 20, stdin), N = getint();
    insert(0, 0);
    for (max = 0, s0 = 0, i = 1; i <= N; i++, s0 = s) {
//      scanf("%d", &d), s = s0 + d;
		fgets(p=buf, 20, stdin), s = s0 + getint();
        if ((k = insert(i, s)) >= 0) {
            if (i - k > max) max = i - k;
        }
    }
    printf("%d\n", max);
    return 0;
}