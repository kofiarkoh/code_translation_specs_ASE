#include <stdio.h>

int main(void) {
	int n,m,p;
	while(scanf("%d%d%d",&n,&m,&p)==3 && (n|m|p)!=0) {
		int t;
		int i,j,k,l;
		int board[7][7][7]={{{0}}};
		int result=0;
		int end_turn=0;
		for(t=0;t<p;t++) {
			int x,y;
			if(scanf("%d%d",&x,&y)!=2)return 1;
			x--;
			y--;
			if(result!=0)continue;
			for(i=0;i<n;i++) {
				if(board[x][y][i]==0) {
					board[x][y][i]=t%2+1;
					break;
				}
			}
			for(i=0;i<n;i++) {
				for(j=0;j<n;j++) {
					for(k=0;k<n;k++) {
						int dx,dy,dz;
						if(board[i][j][k]==0)continue;
						for(dx=-1;dx<=1;dx++) {
							for(dy=-1;dy<=1;dy++) {
								for(dz=-1;dz<=1;dz++) {
									if((dx|dy|dz)==0)continue;
									for(l=1;l<m;l++) {
										int xx=i+dx*l;
										int yy=j+dy*l;
										int zz=k+dz*l;
										if(xx<0 || n<=xx || yy<0 || n<=yy || zz<0 || n<=zz) {
											break;
										}
										if(board[xx][yy][zz]!=board[i][j][k]) {
											break;
										}
									}
									if(l>=m) {
										result=board[i][j][k];
										end_turn=t+1;
									}
								}
							}
						}
					}
				}
			}
		}
		if(result==0) {
			puts("Draw");
		} else if(result==1) {
			printf("Black %d\n",end_turn);
		} else if(result==2) {
			printf("White %d\n",end_turn);
		} else {
			puts("ERROR");
		}
	}
	return 0;
}