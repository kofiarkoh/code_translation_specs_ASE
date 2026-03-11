#include<stdio.h>

int main()
{
	int max, num, i;
	long long count= 0, sum= 0;

	scanf("%d", &max);

	for(i= 0; i< max; i++)
	{
		scanf("%d", &num);
		if(num)
			sum+= num;
		else
		{
			count+= sum/ 2;
			sum= 0;
		}
	}

	if(num)
		count+= sum/ 2;

	printf("%lld\n", count);

	return 0;
}