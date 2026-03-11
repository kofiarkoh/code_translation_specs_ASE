#include <stdio.h>
#include <ctype.h>
#include <string.h>

const char* character_list[38]={
	"a","b","c","d","e","f","g","h","i","j","k","l","m",
	"n","o","p","q","r","s","t","u","v","w","x","y","z",
	"ld","mb","mp","nc","nd","ng","nt","nw","ps","qu","cw","ts"
};

int getnextcode(const char* str) {
	if(str[0]=='l' && str[1]=='d')return 27;
	if(str[0]=='m' && str[1]=='b')return 28;
	if(str[0]=='m' && str[1]=='p')return 29;
	if(str[0]=='n' && str[1]=='c')return 30;
	if(str[0]=='n' && str[1]=='d')return 31;
	if(str[0]=='n' && str[1]=='g')return 32;
	if(str[0]=='n' && str[1]=='t')return 33;
	if(str[0]=='n' && str[1]=='w')return 34;
	if(str[0]=='p' && str[1]=='s')return 35;
	if(str[0]=='q' && str[1]=='u')return 36;
	if(str[0]=='c' && str[1]=='w')return 37;
	if(str[0]=='t' && str[1]=='s')return 38;
	if(islower(str[0]))return str[0]-'a'+1;
	return 0;
}

int main(void) {
	char input[100];
	int table[39][39]={0};
	int n,i,j,length;
	int prev,now;
	int max,maxc;
	scanf("%d ",&n);
	for(i=0;i<n;i++) {
		fgets(input,sizeof(input),stdin);
		length=strlen(input);
		prev=0;
		for(j=0;j<length;) {
			now=getnextcode(&input[j]);
			table[prev][now]++;
			if(now>26)j+=2; else j++;
			prev=now;
		}
	}
	for(i=1;i<=38;i++) {
		max=0;
		maxc=1;
		for(j=1;j<=38;j++) {
			if(table[i][j]>max) {
				max=table[i][j];
				maxc=j;
			}
		}
		printf("%s %s %d\n",character_list[i-1],character_list[maxc-1],max);
	}
	return 0;
}