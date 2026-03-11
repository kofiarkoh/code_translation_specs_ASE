#include<stdio.h>
#include<math.h>

int main(){
	int h1, h2, m1, m2, k;
	int time,ans;
	scanf("%d %d %d %d %d", &h1, &m1, &h2, &m2, &k);
	time = h2 * 60 + m2 - h1 * 60 + m1;
	ans = time - k;

	printf("%d\n", ans);

	return 0;
}