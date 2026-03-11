#include <stdio.h>
#include<string.h>

int main(void)
{
	int i,count=0;
	char s[7];
	scanf("%s",s);
	
	for(i=0;i<6;i++){
		if(s[i]=='1'){
			count++;
		}
	}
	
	printf("%d\n",count);
	return 0;
}