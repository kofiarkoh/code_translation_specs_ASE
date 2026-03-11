#include<stdio.h>
#include<string.h>
int main(){
	char S[1000001];
	char b;
	int zero=0,one=0,seq=0;
	int max=0,M=0;
	scanf("%s",S);
	if(S[0]=='0')zero++;
	else if(S[0]=='1')one++;
	else{
		if(S[1]!='0')zero++;
		else if(S[1]=='0')one++;
	}
	if(zero>one)max=zero-one;
	else max=one-zero;
	if(M<max)M=max;
	for(int i=1;i<strlen(S);i++){
		if(S[i]=='0')zero++;
		else if(S[i]=='1')one++;
		else{
			if(zero>one){
				S[i]='1';
				one++;
			}
			else{
				zero++;
				S[i]='0';
			}
		}
		if(zero>one)max=zero-one;
		else max=one-zero;
		if(M<max)M=max;
	}
	b=S[0];
	seq=1;
	for(int i=1;i<strlen(S);i++){
		if(S[i]==b)seq++;
		else{
			if(seq>M)M=seq;
			b=S[i];
			seq=1;
		}
	}
	if(seq>M)M=seq;
	printf("%d\n",M);
	return 0;
}