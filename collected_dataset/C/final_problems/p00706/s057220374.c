#include <stdio.h>
#define MAX(a,b) ((a) > (b) ? (a) : (b));
int countTree(int a,int b,int c,int d,int e[101][101]);


int main(){
	int N,H,W,S,T,x,y;
	int i,j;
	while(1){
		int maxNumber=0;
		scanf("%d",&N); if(N==0) break;
		scanf("%d%d",&W,&H);
		int map[101][101]={};
		for(i=0;i<N;i++){
			scanf("%d%d",&x,&y);
			map[y][x]=1;
		}
		scanf("%d%d",&S,&T);
		for(i=1;i<=H-T+1;i++){
			for(j=1;j<=W-S+1;j++) maxNumber=MAX(maxNumber,countTree(i,j,T,S,map));
		}
		printf("%d\n",maxNumber);
	}
	return 0;
}

int countTree(int a,int b,int c,int d,int e[101][101]){
	int k,l;
	int countSum=0;
	//printf("[%d,%d]",b,a);
	for(k=a;k<a+c;k++){
		for(l=b;l<b+d;l++){
		if(e[k][l]==1) countSum++;
		}
	}
	//printf("%d\n",countSum);
	return countSum;
}