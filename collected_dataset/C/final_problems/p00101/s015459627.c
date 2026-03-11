#include <stdio.h>
#include <string.h>

int main(int argc, char *argv[]){
	char *p,eng[1002];
	int n;
	scanf("%d ",&n);
	while(n--){
	      	fgets(eng,1002,stdin);
		while(1){
			p=strstr(eng,"Hoshino");
			if(p != NULL) *(p+6)='a';
			else break;
		}
		printf("%s",eng);
	}
	return 0;
}