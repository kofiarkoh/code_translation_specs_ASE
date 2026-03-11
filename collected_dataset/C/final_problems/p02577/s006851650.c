#include <stdio.h>
#include <string.h>
char n[2000000];
int main(void){
	int i,j=0,t;
	scanf("%s",n);
	t=strlen(n);
	for(i=0;i<t;i++)
	j+=n[i]-'0';
	if(j%9==0)
	printf("Yes\n");
	else
	printf("No\n");
	return 0;
}