#include<stdio.h>
#include<string.h>
int main()
{
	char s[20];
	//for(int i=0;i<4;i++) scanf("%d",&s[i]);
	scanf("%s",s);
	if(strlen(s)<4)
	{
		printf("No");
		return 0;
	}
	char check[]="YAKI";
	if(strncmp(s,check,4)==0)
	{
		printf("Yes");
	}
	else
	{
		printf("No");
	}
	return 0;
}