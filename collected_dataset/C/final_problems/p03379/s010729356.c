#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#define Rep(i,a,n)for(int i=(int)(a);i<(int)(n);i++)
#define rep(i,n)Rep(i,0,n)
int up(const void* a, const void* b) {
	return *(int*)a - *(int*)b;
}

int main() {
	int n;
	scanf("%d\n",&n);
	int x[n],y[n],MED,med;
	rep(i, n) { scanf("%d", &x[i]); y[i] = x[i]; }
	qsort(y, n, sizeof(int), up);
	med = y[n / 2-1];
	MED = y[n / 2];
	rep(i, n) {
		if (x[i] <= med)printf("%d\n", MED);
		else printf("%d\n", med);
	}
}