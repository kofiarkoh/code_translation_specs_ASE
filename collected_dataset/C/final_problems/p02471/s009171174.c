#include<stdio.h>
int gcd(int a,int b)
{
	if (a%b == 0)
		return b;
	else
		return gcd(b, a%b);
}

int main()
{
	/*
		ax + by = gcd(a,b)
		y=(gcd(a,b) - ax)/b
	*/
	int a, b;
	scanf("%d %d", &a, &b);
	int g = gcd(a, b);
	int x, y;
	for (x = 0;; x++)
	{
		if ((g + a*x) % b == 0)
		{
			x = -x;
			y = (g - a*x) / b;
			break;
		}
		if ((g - a*x) % b == 0)
		{
			y = (g - a*x) / b;
			break;
		}
		
	}
	printf("%d %d\n", x, y);
	return 0;
}