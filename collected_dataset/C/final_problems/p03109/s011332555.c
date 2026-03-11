#include<stdio.h>

int main(void)
{
	int m;
	char s[11];
	scanf("%s",s);
	m=(s[5]-'0')*10+s[6]-'0';
	if(m<=4)
		printf("Heisei");
	else
		printf("TBD");
	return 0;
}