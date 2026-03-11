#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct s_data
{
	char *str;
	int i;
	int index;
}	t_data;

int data_sort(const void *a, const void *b)
{
	int r = strcmp((*(t_data * )a).str,(*(t_data  *)b).str);
	//printf("%d\n",r);
	if (r == 0)
		return ((*(t_data *)b).i - (*(t_data *)a).i);
	else
		return (r);
}


int main(void)
{
	int n;
	scanf("%d",&n);
	int i =0;
	char **str;
	int *p;
	str = (char **)malloc(sizeof(char*)*(n+2));
	p = (int *)malloc(sizeof(int)*(n+2));
	t_data *data;
	data = (t_data *)malloc(sizeof(t_data)*(n));
	while (i < n)
	{
		str[i] = (char *)malloc(sizeof(char)*(15));
		scanf("%s %d",str[i],&p[i]);
		data[i].str = str[i];
		data[i].i = p[i];
		data[i].index = i+1;
		i++;
	}
	qsort((void *)data, n, sizeof(data[0]),data_sort);
	i = 0;
	while (i < n)
	{
		printf("%d\n",data[i].index);
		i++;
	}
	return (0);
}
