#include <stdio.h>
#include <ctype.h>

char data[8][2][33] = {
	{"\',.?CDFHIKLOSTW"," -ABEGJMNPQRUVXYZ"},
	{" \',-?ABDGJLMNOQRSTUVWXYZ",".CEFHIKP"},
	{"\',-.?ABCDEFGHJMNQRUVWXYZ"," IKLOPST"},
	{"\',.?FHKLORW","-ABCDGIJMNQSTUVXYZ"},
	{"\'-.?AHLSXYZ",",BFGJMNOQTUVW"},
	{"\'-BGHJMWXYZ",",.?ANQUV"},
	{"-JMUVZ","BGNQXY"},
	{"BJNUXZ","-GMQVY"},
};

char data2[7] = " .,-\'?";

int main(){
	int i,j,k,l,p,tmp;
	char str[101];

	while(gets(str) != NULL){
		p = 0;
		tmp = 0;
		for(i = 0;str[i] != '\0';i++){
			for(j = 0;j < 8;j++){
				for(k = 0;k < 2;k++){
					for(l = 0;data[j][k][l] != '\0';l++){
						if(data[j][k][l] == str[i]){
							tmp *= 2;
							tmp += k;
							p++;
							if(p == 5){
								if(tmp < 26){
									putchar(tmp+'A');
								}else{
									putchar(data2[tmp-26]);
								}
								p = 0;
								tmp = 0;
							}
							goto ok;
						}
					}
				}
				ok:;
			}
		}
		if(p){
			for(i = p;i < 5;i++){
				tmp *= 2;
				tmp += 0;
			}
			if(tmp < 26){
				putchar(tmp+'A');
			}else{
				putchar(data2[tmp-26]);
			}
		}
		putchar('\n');
	}

	return 0;
}