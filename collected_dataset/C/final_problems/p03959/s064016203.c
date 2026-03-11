#include<stdio.h>
#include<math.h>

int main()
{
	int num, record[2][100000], i, j;
	long long ans= 1;

	scanf("%d", &num);
	for(i= 0; i< 2; i++)
	{
		for(j= 0; j< num; j++)
			scanf("%d", &record[i][j]);
	}

	if(record[0][num- 1]!= record[1][0])
	{
		printf("0\n");
		return 0;
	}

	for(i= 1; i< num- 1; i++)
	{
		if(record[0][i]== record[0][i- 1] && record[1][i]== record[1][i+ 1])
			if(record[0][i]> record[1][i])
				ans*= record[1][i];
			else
				ans*= record[0][i];
	}

	printf("%lld\n", ans% (long long)(pow(10, 9)+ 7));

	return 0;
}