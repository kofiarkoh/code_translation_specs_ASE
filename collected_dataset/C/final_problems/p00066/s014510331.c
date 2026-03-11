#include<stdio.h>


int main() 
{
	int i;
	char str[10] = {0};

	while (scanf("%s",str)!=EOF)
	{
		//printf("%s\n", str);
		if (str[0]!='s' && str[0] == str[1] && str[1] == str[2])
		{
			i = 0;
		}
		else if (str[0] != 's' && str[0] == str[3] && str[3] == str[6])
		{
			i = 0;
		}
		else if (str[0] != 's' && str[0] == str[4] && str[4] == str[8])
		{
			i = 0;
		}
		else
		{
			if (str[1] != 's' && str[1] == str[4] && str[4] == str[7])
			{
				i = 1;
			}
			else
			{
				if (str[2] != 's' && str[2] == str[5] && str[5] == str[8])
				{
					i = 2;
				}
				else if (str[2] != 's' && str[2] == str[4] && str[4] == str[6])
				{
					i = 2;
				}
				else
				{
					if (str[3] != 's' && str[3] == str[4] && str[4] == str[5])
					{
						i = 3;
					}
					else
					{
						if (str[6] != 's' && str[6] == str[7] && str[7] == str[8])
						{
							i = 6;
						}
						else
						{
							i = 9;
						}
					}
				}
			}
		}
		if (i == 9)
		{
			printf("d\n");
		}
		else
		{
			if (str[i] == 'o')
			{
				printf("o\n");
			}
			else
			{
				printf("x\n");
			}
		}
		
	}

	
		
	
	
	return 0;
}
