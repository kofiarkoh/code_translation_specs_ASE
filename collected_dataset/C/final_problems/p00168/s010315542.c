 #include <stdio.h>

int n;

int saiki(int ima)
{
	if (ima == n)
	{
		return 1;
	}

	if (ima > n)
	{
		return 0;
	}
	
	int res=0;

	res +=saiki(ima+1)+saiki(ima+2)+saiki(ima+3);
	return res; 
}
int main()
{
	int x = saiki(0);
	for(;;)
	{
		scanf("%d", &n);
		if (n==0)
		{
			break;
		}
		printf("%d\n", saiki(0)/ 10 / 365 +1);

	}

	

	
}