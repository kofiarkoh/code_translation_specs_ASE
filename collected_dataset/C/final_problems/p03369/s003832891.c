#include<stdio.h>
#include<string.h>
int main ()
{
	char s[1000];
	int c=0,i;
	scanf("%s",s);
	for(i=0;i<3;i++){
		if (s[i]=='o'){
			c++;
		}
	}
	printf("%d\n",700+(100*c));
	return 0;
}