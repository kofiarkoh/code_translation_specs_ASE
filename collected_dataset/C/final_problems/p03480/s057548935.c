#include<stdio.h>
#include<string.h>
	int main()
	{

	int i,a;
	int count=0;
	char s[100];
	scanf("%s",s);
	a=strlen(s);
	for(i=0;i<a;i++)
{
	if(s[i]=='0')
	{
		count++;
	}
}
printf("%d\n",count);
}