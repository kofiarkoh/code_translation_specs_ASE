// Aizu Vol-1 0113: Period
// 2017.10.23 bal4u@uu

#include <stdio.h>
#include <string.h>

char buf[130], *p;
int getInt(void)
{
	int n;
	if (*p == ' ') p++;
	n = 0; while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

#define MAX 1000000
int rr[MAX+2];
char sps[100], hat[100];

int main()
{
	int a, b, r, k;
	int i, w;

	memset(sps, ' ', sizeof(sps)), memset(hat, '^', sizeof(hat));
	while (fgets(p=buf, 128, stdin) != NULL) {
		a = getInt(), b = getInt();
		memset(rr, -1, sizeof(int)*b);
		for (rr[a] = 0, k = 1; ; k++) {
			a *= 10;
			putchar('0' + a / b);
			if ((r = a % b) == 0) { putchar('\n'); break; }
			if (rr[r] >= 0) {
				putchar('\n');
				i = w = rr[r];
				sps[i] = 0, printf(sps), sps[i] = ' ';
				hat[k-w] = 0, puts(hat), hat[k-w] = '^';
				break;
			}
			rr[r] = k, a = r;
		}
	}
	return 0;
}