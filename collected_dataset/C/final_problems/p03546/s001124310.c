#include <stdio.h>

int c[10][10];
int A[205][205];

int min(int a,int b){
	if(a<=b){return a;}
	else{return b;}
}

void warshall_floyd(){

	int i,j,k;

	for(k=0;k<=9;k++){
		for(i=0;i<=9;i++){
			for(j=0;j<=9;j++){
				c[i][j]=min(c[i][j],c[i][k]+c[k][j]);
			}
		}
	}

}

int main(){

	int H,W;

	scanf("%d %d",&H,&W);

	int i,j;


	for(i=0;i<=9;i++){
		for(j=0;j<=9;j++){
			scanf(" %d",&c[i][j]);
		}
	}

	warshall_floyd();

	int ans=0;

	for(i=0;i<H;i++){
		for(j=0;j<W;j++){
			scanf(" %d",&A[i][j]);
			if(A[i][j]!=-1&&A[i][j]!=1){
			ans+=c[A[i][j]][1];
			}
		}
	}
	printf("%d\n",ans);

	return 0;
}