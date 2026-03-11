// AOJ 2058: Moduic Squares
// 2017.11.6 bal4u@uu

#include <stdio.h>
#include <string.h>
#define TM 0
#if TM
#include <time.h>
#endif

int a[10];
int mk[11];

int check()
{
	int k = (a[0]+a[1]+a[2]) % a[9];
	return  (a[3]+a[4]+a[5]) % a[9] == k &&
            (a[6]+a[7]+a[8]) % a[9] == k &&
            (a[0]+a[3]+a[6]) % a[9] == k &&
            (a[1]+a[4]+a[7]) % a[9] == k &&
            (a[2]+a[5]+a[8]) % a[9] == k &&
            (a[0]+a[4]+a[8]) % a[9] == k &&
            (a[2]+a[4]+a[6]) % a[9] == k;
}

int rec(int i)
{
	int j, r;

	if (i == 10) return check();
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
	int i, v, s;
#if TM
	clock_t start, end;
	start = clock();
#endif
	while (1) {
		memset(mk, 0, sizeof(mk));
		for (s = 0, i = 0; i < 10; i++) {
			scanf("%d", &v); if (v < 0) goto done;
			a[i] = v, mk[v] = 1, s += v;
		}
		if (s == 0) puts("365896");
		else printf("%d\n", rec(0));
	}
done:;
#if TM
	end = clock();
	printf("time %lf\n", (double)(end-start)/CLOCKS_PER_SEC);
#endif
	return 0;
}