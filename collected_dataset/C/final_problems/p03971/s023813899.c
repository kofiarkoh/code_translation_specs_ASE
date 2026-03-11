#include <stdio.h>
#include <stdlib.h>

int	main()
{
	int		N, A, B;
	int		count, ret, sum, lim, i = 0;
	char	*S;

	scanf("%d %d %d\n", &N, &A, &B);
	lim = A + B;
	count = 0;
	sum = 0;
	S = (char *)malloc(sizeof(char) * (N + 1));
	scanf("%s\n", S);
	while (i < N)
	{
		ret = 0;
		if (S[i] == 'a')
		{
			if (sum < lim)
				ret = 1;
		}
		else if (S[i] == 'b')
		{
			if (sum < lim)
			{
				if (count < B)
				{
					ret = 1;
					count++;
				}
			}
		}
		if (ret && ++sum)
			printf("Yes\n");
		else
			printf("No\n");
		i++;
	}
	return (0);
}