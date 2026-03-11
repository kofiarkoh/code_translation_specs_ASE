#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int i,j,n,m;
int wall[200][3],tx,ty,sx,sy;

int crosscheck(int tx,int ty,int sx,int sy,int wx,int wy,int rr){

	int abx=tx-sx,aby=ty-sy;
	int awx=wx-sx,awy=wy-sy;
	int bwx=wx-tx,bwy=wy-ty;

	int rr2=rr*rr;
	int aw2=awx*awx+awy*awy;
	int bw2=bwx*bwx+bwy*bwy;
	int ab2=abx*abx+aby*aby;

	int tim=abx*awy-aby*awx;
	int abw=abx*awx+aby*awy;

	if((aw2-rr2)*(bw2-rr2)<0)return 1;
	if(aw2<rr2 && bw2<rr2)return 0;
	if(abw<0)return 0;
	if(abw>ab2)return 0;

	if( tim*tim/ab2 <= (double)rr2 + 0.00001)return 1;

	return 0;
}

int main(){
	while(scanf("%d",&n)*n){

		for(i=0;i<n;i++)scanf("%d %d %d",&wall[i][0],&wall[i][1],&wall[i][2]);

		for(scanf("%d",&m);m>0;m--){
			scanf("%d %d %d %d",&tx,&ty,&sx,&sy);
			int sss=0;
			for(i=0;i<n;i++){
				int wx=wall[i][0],wy=wall[i][1],rr=wall[i][2];
				if(crosscheck(tx,ty,sx,sy,wx,wy,rr))break;
			}
			if(i<n)printf("Safe\n"); else printf("Danger\n");
		}
	}
	return 0;
}