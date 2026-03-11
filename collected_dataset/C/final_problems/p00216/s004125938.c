#include <stdio.h>
int main(void){
	int n,sum;
	char str[1000];

	for( ; ; ){
		sum=1150;
		fgets(str,sizeof(str),stdin);
		sscanf(str,"%d",&n);
		if(n==-1) break;
		if(n>30){sum += (n - 30)*160; n = 30;}
		if(n>20){sum += (n - 20)*140; n = 20;}
		if(n>10) sum += (n - 10)*125;
		printf("%d\n",4280-sum); 
	} 
	return 0;
}			