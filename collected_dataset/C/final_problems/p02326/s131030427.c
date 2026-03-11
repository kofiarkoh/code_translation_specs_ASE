#include<stdio.h>
#define min(x,y) ((x)<(y) ? (x):(y))
#define N_MAX 1400
int A[N_MAX][N_MAX],B[N_MAX][N_MAX];

int main(void){
    int A[100][100],B[100][100];
    int h,w;
    int i,j,max=0;

    scanf("%d%d",&h,&w);
    for(i=0;i<h;i++) for(j=0;j<w;j++){
	scanf("%d",&A[i][j]);
    }

    for(i=0;i<h;i++){
	if(A[i][0]==1) B[i][0]=0;
	else B[i][0]=1;
    }
    for(j=0;j<w;j++){
	if(A[0][j]==1) B[0][j]=0;
	else B[0][j]=1;
    }

    for(i=1;i<h;i++){
	for(j=1;j<w;j++){
	    if(A[i][j]==1) B[i][j]=0;
	    else B[i][j]=min(min(B[i-1][j],B[i-1][j-1]),B[i][j-1])+1;
	    if(B[i][j]>max) max=B[i][j];
	}
    }

    printf("%d\n",max*max);

    return 0;
}