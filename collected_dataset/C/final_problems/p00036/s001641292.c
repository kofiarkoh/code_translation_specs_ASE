#include <stdio.h>

int main(void){
	int i,j,flag=0,b[11][11]={0};
	char c[11][11]={0};
	
	/*
	while(scanf("%c",&c[0][0])!=EOF){
		flag=0;
		for(i=1;i<8;i++)scanf("%c",&c[0][i]);
		getchar();
		for(i=1;i<8;i++){
			for(j=0;j<8;j++){
				scanf("%c",&c[i][j]);
			}
			getchar();
		}getchar();
		*/
	
	while(scanf("%s",c[0])!=EOF){
		flag=0;
		for(i=1;i<8;i++){
			scanf("%s",c[i]);
		}
		for(i=0;i<8;i++){
			for(j=0;j<8;j++){
				b[i][j]=c[i][j]-'0';
			}
		}
		for(i=0;i<8;i++){
			for(j=0;j<8;j++){
				if(b[i][j]==1){
					if(b[i+1][j]&&b[i][j+1]&&b[i+1][j+1]){
						puts("A");
						flag=1;
						break;
						break;
					}
					else if(b[i+1][j]&&b[i+2][j]&&b[i+3][j]){
						puts("B");
						flag=1;
						break;
						break;
					}
					else if(b[i][j+1]&&b[i][j+2]&&b[i][j+3]){
						puts("C");
						flag=1;
						break;
						break;
					}
					else if(b[i][j+1]&&b[i+1][j+1]&&b[i+1][j+2]){
						puts("E");
						flag=1;
						break;
						break;
					}
					else if(b[i+1][j]&&b[i+1][j+1]&&b[i+2][j+1]){
						puts("F");
						flag=1;
						break;
						break;
					}
					else if(b[i+1][j]&&b[i+1][j-1]&&b[i+2][j-1]){
						puts("D");
						flag=1;
						break;
						break;
					}
					else if(b[i][j+1]&&b[i+1][j-1]&&b[i+1][j]){
						puts("G");
						flag=1;
						break;
						break;
					}
				}
			}
			//if(flag)break;
		}
	}
	
	return 0;
}