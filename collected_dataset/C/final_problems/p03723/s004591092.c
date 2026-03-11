#include <stdio.h>

int main(void)
{
	int a,b,c;
	int count = 0;
	scanf("%d%d%d", &a,&b,&c);
	int tmp_a = a, tmp_b = b, tmp_c = c;
	if (a % 2== 0 && b % 2 == 0 && c % 2 == 0)
	{
		if (a == b && b == c && b == c)
		{
			printf("-1");
			return 0;
		}
	}
	while (a % 2 == 0 && b % 2 == 0 && c % 2 == 0)
	{
		a = (tmp_b / 2) + (tmp_c / 2);
		b = (tmp_c / 2) + (tmp_a / 2);
		c = (tmp_b / 2) + (tmp_a / 2);
		tmp_a = a;
		tmp_b = b;
		tmp_c = c;
		count++;
	}
	printf("%d", count);
	return 0;
}
