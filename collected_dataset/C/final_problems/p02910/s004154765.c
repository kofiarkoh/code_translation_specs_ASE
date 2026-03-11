#include<stdio.h>
int main(){
	char s[101];
	int i=0;
	gets(s);
	while((i%2==0?s[i]!='L':s[i]!='R' )&& s[i++]!='\0');
	if(s[i-1]=='\0')
		printf("Yes");
	else
		printf("No");
	return 0;
} 