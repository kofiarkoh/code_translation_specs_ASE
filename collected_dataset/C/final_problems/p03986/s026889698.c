#include<stdio.h>
#include<string.h>
int main(void){
	int l,s=0,t=0,i;
	char m[262144];
	scanf("%s",m);
	l=strlen(m);
	for(i=0;i<l;i++){
		if(m[i]=='S'){
			s++;
		}
		else if(m[i]=='T'){
			if(s){
				s--;
			}
			else{
				t++;
			}
		}
	}
	printf("%d\n",s+t);
	return 0;
}