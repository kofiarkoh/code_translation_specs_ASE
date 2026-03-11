#include<stdio.h>
int main(void){
	int H,stone[10][5]={0};
	int a,i,j,k,score=0;

	while(1){
		scanf("%d",&H);
		if(H==0){break;}
		score=0;
		for(i=0;i<H;i++){
			scanf("%d %d %d %d %d",&stone[i][0],&stone[i][1],&stone[i][2],&stone[i][3],&stone[i][4]);	
		}
		for(a=0;a<50;a++){
			for(i=0;i<H;i++){
				if(stone[i][0]==stone[i][1]&&stone[i][1]==stone[i][2]&&stone[i][2]==stone[i][3]&&stone[i][3]==stone[i][4]){
					score+=stone[i][2]*5;
					for(j=0;j<5;j++){stone[i][j]=0;}
				}
			}
				
			for(i=0;i<H;i++){
				for(j=0;j<2;j++){
					if(stone[i][j]==stone[i][1+j]&&stone[i][1+j]==stone[i][2+j]&&stone[i][2+j]==stone[i][3+j]){
						score+=stone[i][2]*4;
						for(k=j;k<j+4;k++){stone[i][k]=0;}
					}
				}
			}

			for(i=0;i<H;i++){
				for(j=0;j<3;j++){
					if(stone[i][0+j]==stone[i][1+j]&&stone[i][1+j]==stone[i][2+j]){
						score+=stone[i][2]*3;
						for(k=j;k<j+3;k++){stone[i][k]=0;}
					}
							
				}
			}
			
			for(k=0;k<10;k++){
				for(i=1;i<H;i++){
					for(j=0;j<5;j++){
						if(stone[i][j]==0&&stone[i-1][j]!=0){
							stone[i][j]=stone[i-1][j];
							stone[i-1][j]=0;
						}
					}
				}
			}				
		}
		printf("%d\n",score);
	}
	return 0;
}