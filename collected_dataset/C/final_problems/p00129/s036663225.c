#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int i,j,n,m;
int wall[200][3],tx,ty,sx,sy;

double dist(int px,int py,int qx,int qy,int ax,int ay){
	double vx=qx-px,vy=qy-py,wx=ax-px,wy=ay-py;
	return abs(vx*wy-wx*vy)*abs(vx*wy-wx*vy)/(vx*vx+vy*vy);
	//return (wx*wx+wy*wy)-(vx*wx+vy*wy)*(vx*wx+vy*wy)/(vx*vx+vy*vy);
}

int crosscheck(int tx,int ty,int sx,int sy,int wx,int wy,int rr){
	int abx=sx-tx,aby=sy-ty;
	int awx=wx-tx,awy=wy-ty;
	int bwx=wx-sx,bwy=wy-sy;

	int rr2=rr*rr;
	int aw2=awx*awx+awy*awy;
	int bw2=bwx*bwx+bwy*bwy;
	int ab2=abx*abx+aby*aby;

	int tim=abx*awy-aby*awx;
	int abw=abx*awx+aby*awy;
	int baw=-abx*bwx-aby*bwy;

	if((aw2-rr2)*(bw2-rr2)<0)return 1;
	if(aw2<rr2 && bw2<rr2)return 0;
	if((double)tim*tim/ab2 > (double)rr2)return 0;
	if(abw<0)return 0;
	if(baw<0)return 0;
	return 1;

}

int main(){
	while(scanf("%d",&n)*n){
		for(i=0;i<n;i++)scanf("%d %d %d",&wall[i][0],&wall[i][1],&wall[i][2]);

		for(scanf("%d",&m);m>0;m--){
			scanf("%d %d %d %d",&tx,&ty,&sx,&sy);
			for(i=0;i<n;i++){
				int wx=wall[i][0],wy=wall[i][1],rr=wall[i][2];
				if(crosscheck(tx,ty,sx,sy,wx,wy,rr))break;
			}
			if(i<n)printf("Safe\n"); else printf("Danger\n");
		}
	}
	return 0;
}