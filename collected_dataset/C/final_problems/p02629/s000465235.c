#include<stdio.h>
int main()
{
	long long n,temp,semen,i = 0;
	char input[1005];
	scanf("%lld",&n);
	while(n)
	{
		temp = n / 26;
		semen = n % 26;
		if(semen == 0)
		{
			semen += 26;
			temp--;
		}
		input[i] = semen + 'a' - 1;
		n = temp;
		i++;
	}
	for(int j = i-1;j>=0;j--)
	{
		printf("%c",input[j]);
	}
	return 0;
}