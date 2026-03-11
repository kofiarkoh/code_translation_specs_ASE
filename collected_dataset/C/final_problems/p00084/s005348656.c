#include <stdio.h>
#include <string.h>
int main(void){
	int l=0,f=0;
	char c[10000];
	while(1){
		scanf("%s",c);
		l=strlen(c);
		if(c[l-1]==','||c[l-1]=='.'){
			if(c[l-1]=='.')f=1;
			l=l-1;
			c[l]='\0';
		}
		if(3<=l&&l<=6){
			printf("%s ",c);
		}
		if(f){
			putchar('\b\n');
			break;
		}
	}
	return 0;
}