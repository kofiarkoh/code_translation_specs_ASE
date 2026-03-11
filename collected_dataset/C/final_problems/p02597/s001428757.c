#include<stdio.h>

#define MAX 200010

int main()
{
	int N;
	int i, j;
	int count = 0;

	char str[MAX];

	scanf("%d", &N);
	scanf("%s", str);

	j = N-1;

	for (i = 0 ; i < N ; i++)
	{
		if (str[i] == 'W')
		{
			while(str[j] != 'R')
			{
				j--;
				if (j <= i)
				{
					goto out;
				}
			}
			str[i] = 'R';
			str[j] = 'W';
			count++;
		}
	}

out:
	printf("%d\n", count);

	return 0;
}