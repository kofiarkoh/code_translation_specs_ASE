#include<stdio.h>

int main(void){
	char s[100000];
	int i;
	
	scanf("%s", s);
	
	for(i = 0;s[i];i++){
		if(i % 2 == 0)
			printf("%c", s[i]);
	}
	
	return 0;
}