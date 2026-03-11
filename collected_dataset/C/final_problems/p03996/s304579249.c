#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007LL

int main(void) {
	int a[100010], b[100010], flag, i, j, k, l, M, N, P, Q;
	scanf("%d%d%d", &N, &M, &Q);
	for (i=0;i<Q;i++) scanf("%d", &a[i]);

	for (i=0;i<M;i++) b[i] = M-i;
	if (!memcmp(a, b, sizeof(int)*Q)) {
		printf("Yes\n");
		return 0;
	} else {
		P = 0;
		j = 1;
		for (i=0;i<Q;i++) if (a[i] != 1) { k = a[i]; break; }
		for (i=0;i<Q;i++) {
			if (a[i] == j) continue;
			else if (a[i] == k) P++;
			else { flag=0; break; }
			if (P%N == 0) {
				j = k;
				for (l=i+1;l<Q;l++) if (a[l] != j) { k = a[l]; break; }
				flag = 1; P = 0;
			}
		}
		if (P%N == 0 && flag == 1) printf("Yes\n");
		else printf("No\n");
	}

	return 0;
}