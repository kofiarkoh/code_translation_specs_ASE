#include <stdio.h>
#include <string.h>
int main(void){
	
	int i, flag = 1;
	char s[100];
	
	scanf("%s", s);
	
	for(i=1; i<strlen(s); i+=2){
		if(s[i] == 'R')
		flag = 0;
	}
		
	for(i=0; i<strlen(s); i+=2){
		if(s[i] == 'L')
		flag = 0;
	}
		if(flag == 0){
		printf("No\n");
		}else printf("Yes\n");
	return 0;
}