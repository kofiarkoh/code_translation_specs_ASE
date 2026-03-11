#include<stdio.h>
int main(){
	int i,su,n,j;
	char seg[10][7]={"0111111","0000110","1011011","1001111","1100110","1101101","1111101","0100111","1111111","1101111"};
	char current[7];	

	while(1){
		for(i=0;i<7;i++) current[i]='0';
		scanf("%d",&su);
		if(su==-1) break;
		for(i=0;i<su;i++){
			scanf("%d",&n);
			for(j=0;j<7;j++){
				if(current[j]==seg[n][j]) printf("0");
				else printf("1");
				current[j]=seg[n][j];
			}
			printf("\n");
		}
	}
	return 0;
}