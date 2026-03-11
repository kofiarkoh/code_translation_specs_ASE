#include<stdio.h>
#include<string.h>
char s[16];
char s0[16]={"CODEFESTIVAL2016"};
int num;
int main()
{
	int i;
	scanf("%s",s);
	for(i=0;i<=15;i++)
	{
		if(s[i]!=s0[i])
		  num++;
		else continue;
	}
	printf("%d",num);
	return 0;
}