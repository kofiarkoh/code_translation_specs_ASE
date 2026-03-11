#include <stdio.h>
#include <stdlib.h>


//I love math.

int	main(void)
{
	int l, r, d;
	scanf("%d %d %d", &l, &r, &d);
	int i = l;
	int cnt = 0;
	while (i <= r)
	{
		if (i%d == 0)
			cnt++;
		i++;
	}
	printf("%d\n", cnt);
}
