#include<stdio.h>
#include<stdlib.h>

int main()
{
	int n,m;
	int i,j;
	char *a;
	scanf("%d",&n);
	a = (char *) malloc(sizeof(char) * n);
	scanf("%c", a);
	for(i = 0; i < n; i++)
	{
		scanf("%c", a + i);
	}
	m = n;
	for(i = 0; i < n - 1; i++)
	{
		m--;
		//printf("%d, %d, %d\n", a[n-1], a[n-2], a[n-3]);
		for(j = 0; j < m; j++)
		{
			a[n - j - 1] = (char) abs((char) a[n - j - 1] - (char) a[n - j - 2]);
		}
	}
	printf("%d", a[n - 1]);
}