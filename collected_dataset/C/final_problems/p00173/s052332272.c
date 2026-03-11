#include <stdio.h>

int main(void)
{
	char Class[9][16];
	int am, pm;
	int sum[9];
	int money[9];
	int i;
	
	for (i = 0; i < 9; i++){
		scanf("%s%d%d", Class[i], &am, &pm);
		sum[i] = am + pm;
		money[i] = am * 200 + pm * 300;
	}
	
	for (i = 0; i < 9; i++){
		printf("%s %d %d\n", Class[i], sum[i], money[i]);
	}
	
	return (0);
} 