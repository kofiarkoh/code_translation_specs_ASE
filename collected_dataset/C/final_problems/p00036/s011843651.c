#include <stdio.h>


int main(){
	char  c[8][8];
	int i,j;
	int b,b2;
	char ans,p;
	while(1){
		for(i=0; i<8; i++){
			for(j=0; j<8; j++){
				b=scanf("%c",&c[j][i]);
			}
			if(b==EOF)break;
			scanf("%c",&p);
		}
		for(i=0; i<8; i++){
			for(j=0; j<8; j++){
				if(c[i][j]=='1')c[i][j]=1;
				else c[i][j]=0;
			}
		}
		if(b==EOF)break;
		b2=1;
		for(i=0; i<8; i++){
			for(j=0; j<8; j++){
				if(c[j][i]&&i<7&&j<7){
					if(c[j+1][i]&&c[j][i+1]&&c[j+1][i+1]){
						ans='A';
						b2=0;
						break;
					}
				}
				if(c[j][i]&&i<5){
					if(c[j][i+1]&&c[j][i+2]&&c[j][i+3]){
						ans='B';
						b2=0;
						break;
					}
				}
				if(c[j][i]&&j<5){
					if(c[j+1][i]&&c[j+2][i]&&c[j+3][i]){
						ans='C';
						b2=0;
						break;
					}
				}
				if(c[j][i]&&j>0&&i<6){
					if(c[j-1][i+1]&&c[j][i+1]&&c[j-1][i+2]){
						ans='D';
						b2=0;
						break;
					}
				}
				if(c[j][i]&&j<6&&i<7){
					if(c[j+1][i]&&c[j+1][i+1]&&c[j+2][i+1]){
						ans='E';
						b2=0;
						break;
					}
				}
				if(c[j][i]&&j<7&&i<6){
					if(c[j][i+1]&&c[j+1][i+1]&&c[j+1][i+2]){
						ans='F';
						b2=0;
						break;
					}
				}
			}
			if(!b2)break;
		}
		if(b2)ans='G';
		printf("%c\n",ans);
	}


	return 0;
}