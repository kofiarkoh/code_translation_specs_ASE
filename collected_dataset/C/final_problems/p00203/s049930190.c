#include<stdio.h>
	int w,h,xd[]={1,0,-1},p,u[20][20];
	void ret(int x,int y){
		int i,s=x;
		if(h-1<=y)
			p++;
		else if(u[y][x]==2)
			ret(x,y+2);
		else if(u[y][x]==0){
			y++;
			for(i=0;i<3;i++){
				x=s+xd[i];
				if(x>=0&&x<=w-1){
					if(u[y][x]==2&&i==1)
						ret(x,y+2);
					else if(u[y][x]==0)
						ret(x,y);
				}
			}
		}
	}
	int main(){
		int i,j,x,y;
		while(scanf("%d %d",&w,&h),w||h){
			p=0;
			for(i=0;i<h;i++){
				for(j=0;j<w-1;j++)
				scanf("%d ",&u[i][j]);
				scanf("%d",&u[i][w-1]);
			}
			for(i=0;i<w;i++){
				if(u[0][i]==0){
					x=i;
					y=0;
					ret(x,y);
				}
				else if(u[0][i]==2){
					x=i;
					y=2;
					ret(x,y);
				}
			}
			printf("%d\n",p);
		}
		return 0;
	}