#include<stdio.h>
#include<stdlib.h>
//#include<time.h>
void judge_str(int start, int end,int coefficient);
int count, flag, n;
char genome[101];
int c_coef[500000] = { 0 }, start2[500000], end2[500000], count_str = 0;

int main()
{
	int start, end;
	//clock_t START, END;
	while (1)
	{
		for (int i = 0; i < count_str; i++) c_coef[i] = 0;
		count_str = 0;
		scanf("%s %d",genome,&n);
		if (genome[0] == '0'&&n == 0) break;
		//START = clock();
		count = 0;
		flag = 1;
		for (end = 0; genome[end] != '\0'; end++);
		judge_str(0, end, 1);
		if (count-1 < n)printf("0\n");
		//END = clock();
		//printf("%.2f秒かかりました\n", (double)(END - START) / CLOCKS_PER_SEC);
	}
	return 0;
}
void judge_str(int start, int end, int coefficient)
{
	int i, t,brackets=0;

	for (i=0;i<coefficient && flag;i++)
	{
		for (t = start; t < end && flag; t++)
		{
			if ('0' <= genome[t] && genome[t] <= '9')
			{
				c_coef[count_str] = c_coef[count_str] * 10 + genome[t]-'0';
			}
			else if (genome[t] == '(')
			{
				start2[count_str] = t+1;
				t++;
				for (brackets = 1; brackets; t++)
				{
					if (genome[t] == '(')brackets++;
					if (genome[t] == ')')brackets--;
				}
				t--;
				end2[count_str] = t;
				count_str++;
				judge_str(start2[count_str-1],end2[count_str-1],c_coef[count_str-1]);
				
			}
			else
			{
				if (c_coef[count_str] != 0) judge_str(t, t + 1, c_coef[count_str++]);
				else count++;
				 if (count - 1 == n && flag)
				{
					printf("%c\n", genome[t]);
					flag = 0;
				}
				
			}
		}
	}
}

