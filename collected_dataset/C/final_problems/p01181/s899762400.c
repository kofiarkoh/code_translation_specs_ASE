// AOJ 2058: Moduic Squares
// 2017.11.6 bal4u@uu

#include <stdio.h>
#include <string.h>
#define TM 0
#if TM
#include <time.h>
#endif

int a[9], mod;
int mk[11];

int check()
{
	int k = (a[0]+a[1]+a[2]) % mod;
	return  (a[3]+a[4]+a[5]) % mod == k &&
            (a[6]+a[7]+a[8]) % mod == k &&
            (a[0]+a[3]+a[6]) % mod == k &&
            (a[1]+a[4]+a[7]) % mod == k &&
            (a[2]+a[5]+a[8]) % mod == k &&
            (a[0]+a[4]+a[8]) % mod == k &&
            (a[2]+a[4]+a[6]) % mod == k;
}

int rec(int i)
{
	int j, r;

	if (i == 9) return check();
	if (a[i]) return rec(i+1);

	for (r = 0, j = 1; j <= 10; j++) {
		if (mk[j]) continue;
		a[i] = j;
		mk[j] = 1;
		r += rec(i+1);
		mk[j] = 0;
		a[i] = 0;
	}
	return r;
}

int main()
{
	int i, v;
#if TM
	clock_t start, end;
	start = clock();
#endif
	while (1) {
		memset(mk, 0, sizeof(mk));
		for (i = 0; i < 9; i++) {
			scanf("%d", &v); if (v < 0) goto done;
			a[i] = v, mk[v] = 1;
		}
		scanf("%d", &mod); mk[mod] = 1;
		printf("%d\n", rec(0));
	}
done:;
#if TM
	end = clock();
	printf("time %lf\n", (double)(end-start)/CLOCKS_PER_SEC);
#endif
	return 0;
}