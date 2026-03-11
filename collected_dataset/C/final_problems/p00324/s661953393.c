// Aizu Vol-3 0329: Bilateral Trade
// 2017.8.26 bal4u@uu

#include <stdio.h>

#define MAX 200000

#define HASHSIZ 370003
#define BASE 1100000000
typedef struct { int i, s; } HASH;
HASH hash[HASHSIZ + 5], *hashend = hash + HASHSIZ;

int insert(int i, int s)
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

int main()
{
    int N, d;
    int i, s0, s, k, max;

    scanf("%d", &N);
    // memset(hash, 0, sizeof(hash));
    insert(0, 0);
    for (max = s0 = 0, i = 1; i <= N; i++, s0 = s) {
        scanf("%d", &d), s = s0 + d;
        if (d < -1000000000) while(1);
        if ((k = insert(i, s)) >= 0) {
            if (i - k > max) max = i - k;
        }
    }
    printf("%d\n", max);
    return 0;
}