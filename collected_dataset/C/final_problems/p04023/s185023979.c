#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define min(x,y) ((x)>(y))?(y):(x)

int main() {
	int i, j, k, l, N, Q;
	scanf("%d%d", &N, &Q);
	long long min, ord, s[Q], x;
	long long c[10010][N+1];
	for (i=0;i<Q;i++) scanf("%lld", &s[i]);
	min = 1000000000000000000LL;
	for (i=0;i<Q;i++) if (min>s[i]) { min = s[i]; ord = i; }
	for (i=0;i<Q;i++) if ((i<Q-1 && s[i]>=s[i+1]) || (min<s[i] && ord > i)) s[i] = -1;
	j = 0;
	for (i=0;i+j<Q;i++) {
		while (s[i+j] == -1LL) j++;
		if (j>0) s[i] = s[i+j];
		else continue;
	}
	memset(s+Q-j, 0, sizeof(long long)*j);
	Q -= j;
//	for (i=0;i<Q;i++) printf("%lld\n", s[i]);
	min = (N>s[0])?s[0]:N;
	for (i=1;i<=min;i++) {
		c[i-1][0] = i;
		for (j=1;j<=i;j++) c[i-1][j] = 1;
		for (j=i+1;j<=N;j++) c[i-1][j] = 0;
	}
//	for (i=0;i<min;i++) { for (j=0;j<=N;j++) printf("%lld\t", c[i][j]); printf("\n"); }
	j = min;
	for (i=0;i<Q;i++) {
		x = s[i];
		if (x < min) continue;
		c[j][0] = x;
		for (k=1;k<N+1;k++) c[j][k] = 0;
		while (x>0) {
			for (k=j-1;k>=0;k--) if (x>=c[k][0]) break;
			x -= c[k][0];
			for (l=1;l<=N;l++) c[j][l] += c[k][l];
		}
//		for (l=0;l<=N;l++) printf("%lld\t", c[j][l]); printf("\n");
		j++;
	}

	for (l=1;l<=N;l++) printf("%lld\n", c[j-1][l]);

	return 0;
}
