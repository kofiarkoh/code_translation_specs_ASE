#include<stdio.h>
#include<string.h>
int main(){
	int a,abo[4]={0};
	char c[5];
	for(;;){
		if(scanf("%d,%s",&a,&c) == EOF){break;}
		if(strcmp(c,"A") == 0){abo[0]++;}
		if(strcmp(c,"B") == 0){abo[1]++;}
		if(strcmp(c,"AB") == 0){abo[2]++;}
		if(strcmp(c,"O") == 0){abo[3]++;}
	}
	printf("%d\n%d\n%d\n%d\n",abo[0],abo[1],abo[2],abo[3]);
	return 0;
}