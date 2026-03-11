#include <stdio.h>
#include <string.h>

int main(void){
	char text[1002],i,n;
	fgets(text,1002,stdin);
	n=strlen(text);
	for(i=0;i<n-5;i++){
		if(text[i]=='a' && text[i+1]=='p' && text[i+2]=='p' && text[i+3]=='l' && text[i+4]=='e'){
			text[i]='p';text[i+1]='e';text[i+2]='a';text[i+3]='c';text[i+4]='h';
		}
		else if(text[i]=='p' && text[i+1]=='e' && text[i+2]=='a' && text[i+3]=='c' && text[i+4]=='h'){
			text[i]='a';text[i+1]='p';text[i+2]='p';text[i+3]='l';text[i+4]='e';
		}
		if(text[i]=='A' && text[i+1]=='p' && text[i+2]=='p' && text[i+3]=='l' && text[i+4]=='e'){
			text[i]='P';text[i+1]='e';text[i+2]='a';text[i+3]='c';text[i+4]='h';
		}
		else if(text[i]=='P' && text[i+1]=='e' && text[i+2]=='a' && text[i+3]=='c' && text[i+4]=='h'){
			text[i]='A';text[i+1]='p';text[i+2]='p';text[i+3]='l';text[i+4]='e';
		}
	}
	printf("%s",text);
	return 0;
}