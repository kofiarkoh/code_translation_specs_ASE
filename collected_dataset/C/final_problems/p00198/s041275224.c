#include<stdio.h>
int main(void){
	int i,j,k,n;
	char color[8];
	while(scanf("%d",&n),n!=0){
		int uesita[6]={0},yokomen[6]={0},del=0;
		for(i=0;i<n;i++){
			for(j=0;j<6;j++){
				scanf("%s",&color);
				if(j==0 ||j==5)for(k=0;k<8;k++)uesita[i]+=color[k];
				else for(k=0;k<8;k++)yokomen[i]+=color[k];
			}
		}
		for(i=0;i<n;i++){
			if(i==0 || i==5)if(uesita[i]==uesita[i+1])del++;
			else if(yokomen[i]==yokomen[i+1])del++;
		}
		printf("%d\n",del);
	}
	return 0;
}