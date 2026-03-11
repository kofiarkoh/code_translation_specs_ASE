#include<stdio.h>
#include<string.h>
	int i;
	int Ans =0;
	char s[200000];
	int len;

void match(int j){
	if(s[j]=='A'){
		if(s[j+1]=='B'){
			if(s[j+2]=='C')	{
				s[j]='B';
				s[j+1]='C';
				s[j+2]='A';
				Ans++;
				if(j>0){
					if(s[j-1]=='A'){
						match(j-1);
					}
				}
			}
		}
	}
}

int main(void){
	scanf("%s",s);
	len=strlen(s);
	
	i=0;
	while(i+2<len){
		match(i);
		i+=1;
	}
		printf("%d",Ans);
		
	return 0;
}
