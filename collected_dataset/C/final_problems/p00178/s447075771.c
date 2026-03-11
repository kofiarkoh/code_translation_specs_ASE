#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int i,j,k,block[500000][5],hhh,max[5],left,from,time;
int n,d,l,x;

int main(){
	while(scanf("%d",&n)*n){
		hhh=0;left=0;
		for(i=0;i<5;i++)max[i]=0;

		for(;n>0;n--){

			scanf("%d %d %d",&d,&l,&x); x--;
			left+=l;

			if(d==1){
				k=0;j=0;
				for(i=0;i<l;i++){if(max[x+i]>k){k=max[x+i];j=x+i;}}
				for(i=0;i<l;i++){max[x+i]=k+1;block[k][x+i]=1;}
				from=k;time=1;
			}
			else{
				for(i=0;i<l;i++)block[max[x]+i][x]=1;
				from=max[x];
				max[x]+=l;
				time=l;
				j=x;
			}

			if(hhh<max[j])hhh=max[j];

			for(i=0;i<time;i++){
				if(block[from][0]==1 && block[from][1]==1 && block[from][2]==1 && block[from][3]==1 && block[from][4]==1){
					for(j=0;j<5;j++){block[from][j]=0;}
					for(k=from+1;k<hhh+1;k++){
						for(j=0;j<5;j++){
							block[k-1][j]=block[k][j];
						}
					}
					left-=5;hhh--;
				}
				else from++;
			}

			for(j=0;j<5;j++){
				for(k=max[j];k>=0;k--){
					if(block[k][j]==1)break;
				}
				max[j]=k+1;
			}
		}

		for(i=0;i<hhh;i++){
			for(j=0;j<5;j++){
				block[i][j]=0;
			}
		}

		printf("%d\n",left);
	}
	return 0;
}