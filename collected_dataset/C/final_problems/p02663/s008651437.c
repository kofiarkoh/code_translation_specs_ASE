#include <stdio.h>

int main(){
	int h1, h2, m1, m2, k;
	scanf("%d %d %d %d %d", &h1, &m1, &h2, &m2, &k);
	int ans = (m2-m1)+((h2-h1)*60)-k;
	printf("%d\n", ans);
	return 0;
}