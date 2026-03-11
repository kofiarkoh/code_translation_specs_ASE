
#include<stdio.h>
#include<string.h>
int main(void)
{
	int flag = 0, q, i, n, t,len,count=0;
	char koma[100];
	scanf("%s",koma);
	len = strlen(koma);

	for (i = len; i >= 0; i--)
	{
		
		if (koma[i] != koma[i - 1])
		{
			flag = 0;
			count++;
		}
		
		if (koma[i] == koma[i - 1])//一つまえと違うか比較
		{
			flag++;
		}
		

	}
	printf("%d", count-2);
return 0;
}
