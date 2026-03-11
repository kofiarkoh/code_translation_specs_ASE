// AOJ 2039: Space Coconut Crab II
// 2017.11.6 bal4u@uu
//
// a + b + c = T
// a <= b <= c
// a + b > c

#include <stdio.h>
#include <stdlib.h>
#define TM 0
#if TM
#include <time.h>
#endif

#define MAX  15000
char tbl[MAX+1] = {1,1,0,0};		// zero: if prime
int p[1755] = {						// p[29] = 113 <= sqrt(MAX), max sz = 1754
  2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 
 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 
 73, 79, 83, 89, 97,101,103,107,109,113 }; int sz;

void mkprime()
{
	int i, j, k;
	for (i = 0; i <= 29; i++) {
		k = p[i];
		for (j = k<<1; j < MAX; j += k) tbl[j] = 1;
	}
	for (sz = 29, i = 113; i <= MAX; i += 2) if (!tbl[i]) p[sz++] = i;
}

int bsch(int x)
{
	int m, l = 0, r = sz;

    while (l < r) {
        m = (l + r) >> 1;
        if (p[m] <= x) l = m + 1; else r = m;
    }
	return l-1;
}

int main()
{
	int T, i, j, k, m, ans;
	char buf[10];
#if TM
	clock_t start, end;
	start = clock();
#endif
	mkprime();
	while (fgets(buf, 10, stdin) && *buf != '0') {
		T = atoi(buf);
		if (!(T & 1)) {			// even T
			k = (T>>1)-1;
			if (k > 2 && !(k & 1)) putchar('0');
			else                   putchar('1' - tbl[k]);
			putchar('\n');
		} else {				// odd T
			ans = 0;
			for (i = 0; ; i++) {
				m = bsch((T-p[i]) >> 1);
				if (m < i) break;
				for (j = i; j <= m; j++) {
					k = p[i] + p[j];
					if (T-k < p[j]) break;
					if (k < T && (k<<1) > T && !tbl[T-k]) ans++;
				}
			}
			printf("%d\n", ans);
		}
	}
#if TM
	end = clock();
	printf("time %lf\n", (double)(end-start)/CLOCKS_PER_SEC);
#endif
	return 0;
}