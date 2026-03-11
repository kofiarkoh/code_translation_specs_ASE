
#define ll				long long
#define MOD				1000000007

#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef struct den {
	int sw[10];
	int scnt;
	int p;
} den;

int jcnt;
int jew[50];
int ocnt;

FILE *in, *ans;
int tno;

int out(const char *s) {
#ifdef TEST
	char buf[8192];
	fgets(buf, sizeof(buf), ans);
	if (strcmp(buf, s)) {
		printf("err\n");
	}
#else
	printf(s);
#endif
	return 0;
}

int outi(int i) {
	char s[128];
	sprintf(s, "%d\n", i);
	out(s);
	return 0;
}

int outd(double d) {
	char s[128];
	sprintf(s, "%.10lf\n", d);
	out(s);
	return 0;
}

int outll(ll l) {
	char s[128];
	sprintf(s, "%lld\n", l);
	out(s);
	return 0;
}

int geti() {
	int i;
	fscanf(in, "%d", &i);
	return i;
}

ll getll() {
	ll l;
	fscanf(in, "%lld", &l);
	return l;
}

int getstr(char *s) {
	fscanf(in, "%s", s);
	return 0;
}

int sortfnci(const void *v1, const void *v2) {
	int *i1 = (int *)v1;
	int *i2 = (int *)v2;

	if (*i1 > *i2) {
		return 1;
	}
	else if (*i1 < *i2) {
		return -1;
	}

	return 0;
}

int fnc() {
	int i, j, k;

	jcnt = geti();
	ocnt = geti();

	for (i = 0; i < jcnt; i++) {
		jew[i] = geti();
	}

	int max = 0;
	int lcnt;
	if (jcnt < ocnt) {
		lcnt = jcnt;
	}
	else {
		lcnt = ocnt;
	}
	for (i = 0; i < lcnt; i++) {
		int jcnt2 = jcnt - i;
		int ocnt2 = ocnt - i;
		int rcnt;
		if (jcnt2 < ocnt2) {
			rcnt = jcnt2;
		}
		else {
			rcnt = ocnt2;
		}
		for (j = 0; j < rcnt; j++) {
			int jcnt3 = i + j;
			if (jcnt3 == 0) {
				continue;
			}
			int jew2[50];
			if (i > 0) {
				memcpy(jew2, jew, sizeof(int) * i);
			}
			if (j > 0) {
				memcpy(&jew2[i], &jew[jcnt - j], sizeof(int) * j);
			}
			qsort(jew2, jcnt3, sizeof(int), sortfnci);

			int ocnt3 = ocnt2 - j;
			if (ocnt3 > jcnt3) {
				ocnt3 = jcnt3;
			}
			for (k = 0; k < ocnt3; k++) {
				if (jew2[k] < 0) {
					jew2[k] = 0;
				}
				else {
					break;
				}
			}
			int sum = 0;
			for (k = 0; k < jcnt3; k++) {
				sum += jew2[k];
			}
			if (max < sum) {
				max = sum;
			}
		}
	}

	outi(max);

	return 0;
}

int main() {

#ifdef TEST
	for (tno = TEST_SNO; tno <= TEST_ENO; tno++) {
		char str[128];
		sprintf(str, "test%d.txt", tno);
		in = fopen(str, "r");
		sprintf(str, "ans%d.txt", tno);
		ans = fopen(str, "r");
		fnc();
		fclose(in);
		fclose(ans);
	}
	printf("end\n");
#else
	in = stdin;
	fnc();
#endif

	return 0;
}
