#include<stdio.h>

int main()
{
	int e, x, y, z;
	int i, j, min;
	
	while(1)
	{		
		scanf("%d", &e);
		
		if(e == 0) break;
		
		min = -1;
		
		z = 0;
		while(z * z * z <= e)
		{
			z++;
		}
		
		for(i = 0; i < z; i++)
		{
			y = 0;
			while(y * y <= (e - i * i * i))
			{
				y++;
			}
			
			for(j = 0; j < y; j++)
			{
				x = e - i * i * i - j * j;
				if(min < 0 || min > i + j + x)
				{
					min = i + j + x;
				}
			}
		}
		
		printf("%d\n", min);
	}
	
	return 0;
}