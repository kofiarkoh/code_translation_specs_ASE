#include<stdio.h>
int main()
{
	int n;
	char s[100],t[100];
	int i;
	scanf("%d %s %s",&n,s,t);
	for(i=0;i<n;i++){
		printf("%c%c",s[i],t[i]);
	}
	return 0;
}