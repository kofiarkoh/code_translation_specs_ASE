#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int i,j,n,m;
int wall[100][3],tx,ty,sx,sy,vx,vy;

double dist(int px,int py,int qx,int qy,int ax,int ay){
	int vx=qx-px,vy=qy-py,wx=ax-px,wy=ay-py;
	return abs(vx*wy-wx*vy)/sqrt((double)vx*vx+vy*vy);
}

int main(){
	//printf("%lf",dist(0,0,0,10,-1,1));

	while(scanf("%d",&n)*n){
		for(i=0;i<n;i++)scanf("%d %d %d",&wall[i][0],&wall[i][1],&wall[i][2]);

		for(scanf("%d",&m);m>0;m--){
			scanf("%d %d %d %d",&tx,&ty,&sx,&sy);
			vx=sx-tx;vy=sy-ty;
			for(i=0;i<n;i++){
				int wx=wall[i][0],wy=wall[i][1],rr=wall[i][2];
				if(((tx-wx)*(tx-wx)+(ty-wy)*(ty-wy)-rr*rr)*((sx-wx)*(sx-wx)+(sy-wy)*(sy-wy)-rr*rr)<0)break;
				if((tx-wx)*(tx-wx)+(ty-wy)*(ty-wy)<=rr*rr)continue;
				if(fabs(dist(tx,ty,sx,sy,wx,wy))>(double)rr)continue;
				if((sx-tx)*(wx-tx)+(sy-ty)*(wy-ty)<0)continue;
				if((tx-sx)*(wx-sx)+(ty-sy)*(wy-sy)<0)continue;
				break;
			}
			if(i<n)printf("Safe\n"); else printf("Danger\n");
		}
	}
	return 0;
}