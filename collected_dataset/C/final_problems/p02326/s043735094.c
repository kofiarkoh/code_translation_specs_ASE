#include<stdio.h>
#define N_MAX 1400
#define min(a,b) ((a)<(b) ? (a):(b))
#define max(a,b) ((a)>(b) ? (a):(b))
int A[N_MAX][N_MAX],B[N_MAX][N_MAX];
int main(void){
    int h,w;
    int i,j,k;
    int ans=0;

    scanf("%d%d",&h,&w);
    for(i=0;i<h;i++){
	for(j=0;j<w;j++){
	     scanf("%d",&A[i][j]);
	     if(A[i][j]==1) B[i][j]=0;
	     else {
		B[i][j]=1;
		ans=1;
	     }
	}
    }

/*
    for(i=0;i<h;i++){
	for(j=0;j<w;j++){
	    if(j) printf(" ");
	    printf("%d",B[i][j]);
	}
	printf("\n");
    }
*/

    for(i=1;i<h;i++){
	for(j=1;j<w;j++){
	    if(B[i][j]!=0)
		B[i][j]=min(min(B[i-1][j],B[i-1][j-1]),B[i][j-1])+1;
	    ans=max(ans,B[i][j]);
	}
    }

    printf("%d\n",ans*ans);

    return 0;
}