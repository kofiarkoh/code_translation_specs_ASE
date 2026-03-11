#include <stdio.h>

#define MAX	(100)

int main(void)
{
	int data[MAX + 1] = { 0 };
	int most_num = 0;
	int i;
	
	while (scanf("%d", &i) != EOF) data[i]++;
	for (i = 1; i <= MAX; i++) if (most_num < data[i]) most_num = data[i];
	for (i = 1; i <= MAX; i++) if (most_num == data[i]) printf("%d\n", i);
	
	return (0);
}