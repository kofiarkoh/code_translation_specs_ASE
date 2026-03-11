#include<stdio.h>
#include<math.h>
#include<string.h>
#include<memory.h>
#include<stdlib.h>
int comp(const void* a, const void* b)
{
	return *(int*)a - *(int*)b;
}
int main(void)
{
	char s[100005];
	scanf("%s", s);
	int l = strlen(s);
	int i, sc = 0;
	for (i = 0; i < l; i++)
	{
		if (i % 2 == 0)
		{
			if (s[i] == 'p')
				sc--;
		}
		else
		{
			if (s[i] == 'g')
				sc++;
		}
	}
	printf("%d", sc);
	return 0;
}