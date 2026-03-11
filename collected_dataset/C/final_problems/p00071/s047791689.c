#include <stdio.h>
#include <math.h>
int bomb[8][8];
void fire(int i,int j){
	int k;
	bomb[i][j]=0;
	for(k=(i<3?0:i-3);k<(i>4?8:i+4);k++)if(bomb[k][j])fire(k,j);
	for(k=(j<3?0:j-3);k<(j>4?8:j+4);k++)if(bomb[i][k])fire(i,k);
}
int main(void){
	int i,j,n,x;
	scanf("%d",&n);
	for(x=1;x<=n;x++){
		printf("Data %d:\n",x);
		for(i=0;i<8;i++){
			for(j=0;j<8;j++){
				scanf("%1d",&bomb[i][j]);
			}
		}
		scanf("%d %d",&j,&i);
		fire(i-1,j-1);
		for(i=0;i<8;i++){
			for(j=0;j<8;j++){
				printf("%d",bomb[i][j]);
			}
			printf("\n");
		}
	}
	return 0;
}