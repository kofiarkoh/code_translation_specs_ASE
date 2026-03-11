#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int i,j,n,step,set,safe;
int x,y,spr[15][2],mark[15][3][3],mat[30][30];

void check(int x,int y){
	if(abs(spr[step][0]-x)<2 && abs(spr[step][1]-y)<2){
		mark[step][x-spr[step][0]+1][y-spr[step][1]+1]=set;
		safe=1;
	}
}

void marking(int x,int y){
	int k;
	for(k=-1;k<2;k++){
		check(x+2,y+k);
		check(x-2,y+k);
		check(x+k,y+2);
		check(x+k,y-2);
	}
}

int main(){
	while(scanf("%d %d",&y,&x)*(x+y)){
		scanf("%d",&n);
		set++;
		for(i=0;i<n;i++){scanf("%d %d",&spr[i][1],&spr[i][0]);}
		spr[n][1]=spr[0][1];spr[n][0]=spr[0][0];
		for(step=0;step<=n;step++){
			safe=0;
			if(step==0)marking(x,y);
			else{
				for(i=0;i<3;i++){
					for(j=0;j<3;j++){
						if(mark[step-1][i][j]==set)marking(spr[step-1][0]+i-1,spr[step-1][1]+j-1);
					}
				}
			}
			if(safe==0)break;
		}
		if(step>n) printf("OK\n"); else printf("NA\n");
	}
	return 0;
}