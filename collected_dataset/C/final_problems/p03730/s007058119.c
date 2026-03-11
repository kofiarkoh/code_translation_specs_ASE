#include<stdio.h>

int main()
{
	int a,b,c,i;
	scanf("%d %d %d",&a,&b,&c);

	for (i=1;i<=10000;i++) {
		if (a*i % b == c) {
			printf("YES\n");
			return 0;
		}
	}
	printf("NO\n");
	return 0;
}
