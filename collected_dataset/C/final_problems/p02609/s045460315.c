#include <stdio.h>
#include <stdlib.h>

int f_pop(int num)
{
	int r = 0;
	int i = 0;
	while (num != 0)
	{
		if((num&1) == 1)
			r++;
		num = num>>1;
		i++;
	}
	//printf("pop num:%d return:%d\n",num,r);
	return (r);
}

int int_pow(int a, int b)
{
	int i = 0;
	int r = 1;
	while (i < b)
	{
		r = r * a;
		i++;
	}
	return (r);
}

int memo_f(int num, int *memo)
{
	if (num == 0)
		return (0);
	if (num == 1 || num == 2|| num == 4)
		return (1);
	if (memo[num-1] != 0)
		return(memo[num-1]);
	int r;
	//printf("debug101\n");
	int pop = f_pop(num);
	
	//printf("debug102\n");
	r = memo_f((num%pop), memo) + 1;
	memo[num -1] = r;
	//printf("debug103\n");
	return (r);

}

int main(void)
{
	int n;
	scanf("%d",&n);
	char *x;
	int *ans;
	int *memo;
	x = (char *)calloc(sizeof(char),n+1);
	
	ans = (int *)calloc(sizeof(int),n);
	memo = (int *)calloc(sizeof(int),int_pow(2,n+1));
	
	scanf("%s",x);

	int i = 0;
	int int_x = 0;;
	while (i < n)
	{
		int_x = int_x | (x[i]=='0'?0:1);
		int_x = int_x<<1;
		i++;
	}
	int_x = int_x>>1;

	int j = 0;
	i = n-1;
	int tmp;
	//printf("debug0001000\n");
	while (i >= 0)
	{
		int z = int_pow(2,i);
		//printf("debug0001200 j:%d int_x:%d\n",j,int_x);
		ans[j] = memo_f(int_x ^ z, memo);
		//printf("debug0001300\n");
		i--;
		j++;
	}
	int index = 0;
	//printf("debug0002000\n");
	while (index < n)
	{
		printf("%d\n",ans[index]);
		index++;
	}

	//printf("debug0003000\n");
	return (0);
}
