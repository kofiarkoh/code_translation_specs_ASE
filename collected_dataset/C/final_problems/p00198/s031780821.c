#include<stdio.h>
int main(void){
	int i,j,k,n;
	char color[8];
	while(scanf("%d",&n),n>0){
		int uesita[6]={0},yokomen[6]={0},del=0;
		for(i=0,k=-1;i<=n;i++,k++){
			if(i<n){
				for(j=0;j<6;j++){
					scanf("%s",&color);
					if(j==0 ||j==5)uesita[i]+=color[0];
					else yokomen[i]+=color[0];
				}
			}
			if(k>=0){
				if(k==0 || k==5)if(uesita[k]==uesita[k+1])del++;
				else if(yokomen[k]==yokomen[k+1])del++;
			}
		}
		printf("%d\n",del);
	}
	return 0;
}