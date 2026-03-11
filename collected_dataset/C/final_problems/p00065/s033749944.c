#include <stdio.h>
#include <string.h>
int main(void){
	char tex[30];
	int pat[2][1001],p;
	for(p=0;p<1001;p++){
		pat[0][p]=0;pat[1][p]=0;
	}
	while(fgets(tex,30,stdin)!=NULL){
		if(tex[0]=='\n')break;
		sscanf(tex,"%d,%*d",&p);
		pat[0][p]++;
		pat[1][p]=1;

	}
	while(fgets(tex,30,stdin)!=NULL){
		sscanf(tex,"%d,%*d",&p);
		pat[0][p]++;
		if(pat[1][p]==1)pat[1][p]=2;
	}
	for(p=0;p<1001;p++){
		if(pat[1][p]==2)printf("%d %d\n",p,pat[0][p]);
	}
	return 0;
}