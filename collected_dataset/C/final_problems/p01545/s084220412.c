// AOJ 2431: House Moving
// 2017.11.16 bal4u@uu

#include <stdio.h>
#include <stdlib.h>

char buf[600000], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

#define SIZE 100005
int N;
int max;
int num[SIZE+3];
int LIS[SIZE+3];
int tmp[SIZE+3];

void calcLIS(void)
{
    int k;
    int lo, hi, mid;

    LIS[N-1] = 1;
    tmp[0] = num[N-1];
    max = 1;
    for (k = N-2; k >= 0; k--) {
        if (tmp[max-1] > num[k]) { tmp[max++] = num[k]; LIS[k] = max; continue; }
        if (tmp[max-1] == num[k]) { LIS[k] = max; continue; }
        lo = 0, hi = max;
        while (lo < hi) {
            mid = (lo + hi) >> 1;
            if (tmp[mid] > num[k]) lo = mid+1;
            else hi = mid;
        }
        tmp[lo] = num[k], LIS[k] = lo+1;
    }
}

int main()
{
	int i, j;
	long long sum, a;

	fgets(p=buf, 10, stdin), N = getint();
	fgets(p=buf, sizeof(buf), stdin);
	for (sum = 0, i = 0; i < N; i++) sum += (num[i] = getint()), p++;
	calcLIS();
	for (a = 0, j = max, i = 0; i < N; i++) if (LIS[i] == j) a += num[i], j--;
	printf("%lld\n", sum - a);
	return 0;
}