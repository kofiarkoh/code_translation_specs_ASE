#include <stdio.h>

int main(void)
{
	int tyou=0, hisi=0;
	int a, b, c;
	
	while(scanf("%d,%d,%d", &a, &b, &c)!=EOF) {
		if(c*c==a*a+b*b) tyou++;
		if(a==b) hisi++;
	}
	
	printf("%d\n%d\n", tyou, hisi);
	
	return 0;
}
