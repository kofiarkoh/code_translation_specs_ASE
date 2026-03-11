#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int n;
char tex[1050];
char *fin;

int main(void){
	scanf("%d",&n);
	fgets(tex,1001,stdin);
	for(;n>0;n--){
		fgets(tex,1001,stdin);
		while(strstr(tex,"Hoshino")!=NULL){
			fin=strstr(tex,"Hoshino");
			fin[6]='a';
		}
		printf("%s",tex);
	}
	return 0;
}