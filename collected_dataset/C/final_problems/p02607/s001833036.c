#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int compare(const void*a,const void *b) {
	return *(int*)a - *(int*)b;
}

int main() {
	int l,r,d,k = 0, n, a[110] = { 0 }, ans = 0;
	char s[3], cas[4][4] = {"AC","WA","TLE","RE"};
	scanf("%d", &n);
	for (int i = 0; i < n; i++) scanf("%d", &a[i]);
	for (int i = 0; i < n; i++) {
		if (i % 2 == 1 && a[i] % 2 == 1)ans++;
	}
	printf("%d",ans);
	return 0;
}