#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int i,j,k,n,m,l,x,y,build[1001][2],assign[1001],assign2[1001],prev[1001][1001];
double route[2][1001][1001];

void backtrack(int from,int to){
	int z=prev[from][to];
	if(z==from){
		printf("%d ",assign[from]);
		return;
	}
	backtrack(from,z);
	backtrack(z,to);
}

int main(){
	while(scanf("%d",&n)*n){
		for(i=0;i<n;i++){
			scanf("%d %d %d",&assign[i],&build[i][0],&build[i][1]);
			assign2[assign[i]]=i;
		}
		for(i=0;i<n;i++){
			for(j=i;j<n;j++){
				l=(build[i][0]-build[j][0])*(build[i][0]-build[j][0])+(build[i][1]-build[j][1])*(build[i][1]-build[j][1]);
				if(l<=2500){
					route[0][i][j]=sqrt((double)l);
					route[0][j][i]=sqrt((double)l);
					prev[i][j]=i;
					prev[j][i]=j;
				}
				else{
					route[0][i][j]=999999999;
					route[0][j][i]=999999999;
				}
			}
		}

		for(l=1;pow(2.0,l)<n-1;l++){
			for(i=0;i<n;i++){
				for(j=0;j<n;j++){
					route[l%2][i][j]=route[(l+1)%2][i][j];
					for(k=0;k<n;k++){
						if(route[l%2][i][j]>route[(l+1)%2][i][k]+route[(l+1)%2][k][j]){
							route[l%2][i][j]=route[(l+1)%2][i][k]+route[(l+1)%2][k][j];
							prev[i][j]=k;
						}
					}
				}
			}
		}

		l--;

		for(scanf("%d",&m);m>0;m--){
			scanf("%d %d",&x,&y);
			x=assign2[x];
			y=assign2[y];
			if(x==y){
				printf("%d\n",assign[y]);
			}
			else if(route[l%2][x][y]<999999999){
				backtrack(x,y);
				printf("%d\n",assign[y]);
			}
			else printf("NA\n");
		}

	}
	return 0;
}