#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(void){
	char s[100000];
	scanf("%s",s);
	int alp[26];
	for(int i=0;i<strlen(s);i++){
		alp[s[i]-97] = 1;
	}
	for(int i=0;i<26;i++){
		if(alp[i] == 0){
			printf("%c",i+97);
			return 0;
		}
	}
	printf("None");
	return 0;
}