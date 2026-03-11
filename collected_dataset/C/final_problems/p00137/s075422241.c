#include <stdio.h>

int main(void)
{
	int n;
	int s;
	int rund;
	int i;
	int j;
	
	scanf("%d", &n);
	
	for (i = 1; i <= n; i++){
		scanf("%d", &s);
		printf("Case %d:\n", i);
		for (j = 0; j < 10; j++){
			s = s * s / 100 % 10000;
			printf("%d\n", s);
		}
	}
	
	return (0);
}