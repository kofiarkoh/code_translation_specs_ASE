#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int i,r,g,b,d,ans,best;
char col[8][15]={"black","blue","lime","aqua","red","fuchsia","yellow","white"};
char code[24];

int main(){
	while(1){
		scanf("%s",&code);
		if(code[0]=='0')break;
		sscanf(code,"#%2x%2x%2x",&r,&g,&b);
		best=99999999;
		for(i=0;i<8;i++){
			d=(r-255*(i>3))*(r-255*(i>3))+(g-255*(i%4>1))*(g-255*(i%4>1))+(b-255*(i%2))*(b-255*(i%2));
			if(d<best){best=d;ans=i;}
		}
		printf("%s\n",col[ans]);
	}
	return 0;
}