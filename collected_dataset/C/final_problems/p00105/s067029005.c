#include<stdio.h>
#include<string.h>

int main(void)
{
	int num[101], tempn, index[101][31], i, j, k, l, m, alc, count;
	char word[101][31],tempw[31],dict[101][31];

	i = 0;
	while (~scanf("%s %d", word[i], &num[i]))
		i++;
	i = i - 1;
	j = 0;  
	while (j < i)    
	{      
		if (strcmp(word[j], word[j+1]) > 0)
		{
			strcpy(tempw, word[j]);
			strcpy(word[j], word[j+1]);
			strcpy(word[j+1], tempw);
			tempn = num[j];
			num[j] = num[j+1];
			num[j+1] = tempn;      
			j=0;
		}    
		else
			j++;
	}

	l = 0;
	alc = 0;
	count = 0;
	for (k = 0; k <= i; k++)    
	{      
		if (k == 0)
		{      
			strcpy(dict[0], word[0]);
			index[0][0] = num[0];
			l++;
		}      
		else    
		{      
			if (strcmp(dict[l-alc-1], word[k]) != 0)
			{          
				count = 0;
				strcpy(dict[l-alc], word[k]);
				index[l-alc][0] = num[k];
				l++;
			}      
			else
			{
				count++;
				alc++;
				index[l-alc][count] = num[k];
				l++;
			}
		}
	}
	for (k=0;k<i;k++)
	{
		m = 0;
		while (index[k][m+1] != 0)
		{
			if (index[k][m] > index[k][m+1])
			{
				tempn = index[k][m];
				index[k][m] = index[k][m+1];
				index[k][m+1] = tempn;
				m = 0;
			}
			else
				m++;
		}
	}
	for (k = 0; k < l-alc; k++)
	{
		m = 0;
		printf("%s\n", dict[k]);
		while (index[k][m+1] != 0)
		{
			printf("%d ", index[k][m]);
			m++;
		}
		printf("%d\n", index[k][m]);
	}

	return 0;
}