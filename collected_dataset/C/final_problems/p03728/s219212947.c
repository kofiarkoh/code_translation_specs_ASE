#include <stdio.h>
#include <string.h>

int N, p[200010], a[200010];

void highlow(void) {
	int i, max;
	a[0] = 1; max = p[0];
	for (i=1;i<N;i++) {
		if (max<p[i]) { a[i] = 1; max = p[i]; }
		else a[i] = 0;
	}
}

int check(void) {
	int i, f = 1;
	for (i=0;i<N-1;i++) {
		if (!f || p[i]>=p[i+1]) f = 0;
	}
	return 1-f;
}

int main(void) {
	int i, j, M, pn[200010];
	scanf("%d", &N);
	for (i=0;i<N;i++) scanf("%d", &p[i]);

	M = 0;
	while (1) {
		highlow();
		j = 0; memset(pn,0,sizeof(int)*N);
		for (i=0;i<N;i++) {
			if (!a[i]) pn[j++] = p[i];
		}
		for (i=0;i<N;i++) {
			if (a[i]) pn[j++] = p[i];
		}
		memmove(p,pn,sizeof(int)*N);
		M++;
		if (!check()) break;
	}
	printf("%d\n", M);

	return 0;
}