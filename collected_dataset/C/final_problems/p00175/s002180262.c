#include<stdio.h>
int main(void)
{
	int n,nbox[10000],temp,i,j,flg;
	scanf("%d",&n);
	while(n!=-1){
		temp=0;
		i=0;
		flg=0;
		while(n>=4){
			nbox[i]=n%4;
			n/=4;
			i++;
		}
		printf("%d",n);
		for(j=i;j>=0;j--){
			if(flg==0){
				flg=1;
			}
			else if(flg==1){
				printf("%d",nbox[j]);
			}
		}
		printf("\n");
		scanf("%d",&n);
	}
	return 0;
}