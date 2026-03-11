#include<stdio.h>
#include<string.h>

int a,b,i;
char s[120];

int main(){
	scanf("%s",s);
	while(strcmp(s,"0")!=0){
		a=b=0;
		for(i=1;i<strlen(s);i++){
			if(s[i]=='A')a++;
			else b++;
		}
		if(a>b)a++;
		else b++;
		printf("%d %d\n",a,b);
		scanf("%s",s);
	}
	return 0;
}