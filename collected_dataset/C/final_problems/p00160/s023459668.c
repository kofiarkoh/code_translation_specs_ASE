#include<stdio.h>

int f(int p);
int g(int p);

int main(void)
{
	int i;
	int n, x, y, h, w, s;
	while (1)
	{
		scanf("%d", &n);
		if (n == 0)
			break;
		s = 0;
		for (i = 0; i < n; i++)
		{
			scanf("%d%d%d%d", &x, &y, &h, &w);
			if (f(x+y+h) != -1 && g(w) != -1)
				s += f(x+y+h) > g(w) ? f(x+y+h) : g(w);
		}
		printf("%d\n", s);
	}
	return 0;
}

int f(int p)
{
	if (p <= 60)
		return 600;
	else if (p <= 80)
		return 800;
	else if (p <= 100)
		return 1000;
	else if (p <= 120)
		return 1200;
	else if (p <= 140)
		return 1400;
	else if (p <= 160)
		return 1600;
	else
		return -1;
}

int g(int p)
{
	if (p <= 2)
		return 600;
	else if (p <= 5)
		return 800;
	else if (p <= 10)
		return 1000;
	else if (p <= 15)
		return 1200;
	else if (p <= 20)
		return 1400;
	else if (p <= 25)
		return 1600;
	else
		return -1;
}