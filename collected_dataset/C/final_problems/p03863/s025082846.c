#include <stdio.h>
#include <string.h>

char s[100005];

int main(){

	scanf("%s",s);

	int len=strlen(s);

	if(len%2==0){
		if(s[0]==s[len-1]){
			printf("First\n");
		}
		else{
			printf("Second\n");
		}
	}
	else{
		if(s[0]==s[len-1]){
			printf("Second\n");
		}
		else{
			printf("First\n");
		}
	}

	return 0;
}
