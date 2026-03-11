#include <stdio.h>
#include<string.h>

int main(void)
{
	char str[21],strsub[21];
	int i,x;
	scanf("%s",str);
	x = strlen(str);
	for(i=0;i<x;i++)
	{
		strsub[i] = str[x-1-i];
	}
	printf("%s",strsub);
	return 0;
}