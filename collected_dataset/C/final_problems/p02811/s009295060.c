#include<stdio.h>
int main(void)
{
	int k = 0,x = 0;
	scanf("%d%d",&k, &x);

	if (500 * k >= x) {
		printf("Yes\n");
	}
	else {
		printf("No\n");
	}


	return 0;
}