
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

int sw[10];
int scnt;
den dens[10];
int dcnt;
int acnt;

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

int fnc2(int sno) {
	int i, j;

	if (sno >= scnt) {
		for (i = 0; i < dcnt; i++) {
			int ocnt = 0;
			for (j = 0; j < dens[i].scnt; j++) {
				ocnt += sw[dens[i].sw[j] - 1];
			}
			if (ocnt % 2 != dens[i].p) {
				return 0;
			}
		}

		acnt++;

		return 0;
	}

	sw[sno] = 1;
	fnc2(sno + 1);

	sw[sno] = 0;
	fnc2(sno + 1);

	return 0;
}

int fnc() {
	int i, j;

	scnt = geti();
	dcnt = geti();

	for (i = 0; i < dcnt; i++) {
		dens[i].scnt = geti();
		for (j = 0; j < dens[i].scnt; j++) {
			dens[i].sw[j] = geti();
		}
	}
	for (i = 0; i < dcnt; i++) {
		dens[i].p = geti();
	}

	fnc2(0);

	outi(acnt);

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
