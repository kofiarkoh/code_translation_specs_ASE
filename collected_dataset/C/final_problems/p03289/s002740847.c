#include<stdio.h>
#include<string.h>
int main (){
	int a,b,i;
	char str[15];
	scanf("%s",str);
	a=strlen(str);
	if(str[i]!='A'){
		printf("WA\n");
		return 0;
	}
	int count=0;
	for(i=2;i<strlen(str)-1;i++){
		if(str[i]=='C'){
			count++;
			}
	}
	if(count!=1){
		printf("WA\n");
		return 0;
	}
	count=0;
	for(i=0;i<strlen(str);i++){
		if(str[i]>='a' && str[i]<='z'){
			count++;
			}
	}
	if (count!=a-2){
			printf("WA\n");
			return 0;
		}
	printf("AC\n");
	return 0;
}
