#include <stdio.h>
#include <string.h>

int C,D,W,X;
/* [area][day] */
int E[15][30]; 
int F[15][30];

/* [day][area][direction][rest fuka][rest multiple] */
int memo[30][16][3][51][6];

int tansaku(int day,int area,int direction,int rest_fuka,int rest_multiple) {
	int result=0,nowresult;
	int i;
	if(day>=D)return 0;
	if(memo[day][area][direction][rest_fuka][rest_multiple]>0) {
		return memo[day][area][direction][rest_fuka][rest_multiple]-1;
	}
	result=tansaku(day+1,0,0,rest_fuka,rest_multiple);
	if(area==0) {
		for(i=0;i<C;i++) {
			if(E[i][day]>0 && F[i][day]<=rest_fuka) {
				int nownowresult;
				nowresult=tansaku(day+1,0,0,rest_fuka-F[i][day],rest_multiple);
				if(rest_multiple>0) {
					nownowresult=tansaku(day,i+1,0,rest_fuka-F[i][day],rest_multiple-1);
					if(nownowresult>nowresult)nowresult=nownowresult;
				}
				nowresult+=E[i][day];
				if(nowresult>result)result=nowresult;
			}
		}
	} else {
		/* direction : 0 is none, 1 is left, 2 is right */
		if(direction!=2 && area>1 && E[area-2][day]>0 && F[area-2][day]<=rest_fuka) {
			nowresult=tansaku(day,area-1,1,rest_fuka-F[area-2][day],rest_multiple);
			nowresult+=E[area-2][day];
			if(nowresult>result)result=nowresult;
		}
		if(direction!=1 && area<C && E[area][day]>0 && F[area][day]<=rest_fuka) {
			nowresult=tansaku(day,area+1,2,rest_fuka-F[area][day],rest_multiple);
			nowresult+=E[area][day];
			if(nowresult>result)result=nowresult;
		}
	}
	memo[day][area][direction][rest_fuka][rest_multiple]=result+1;
	return result;
}

int main(void) {
	int i,j;
	while(1) {
		scanf("%d%d%d%d",&C,&D,&W,&X);
		if((C|D|W|X)==0)break;
		for(i=0;i<C;i++) {
			for(j=0;j<D;j++) {
				scanf("%d",&E[i][j]);
			}
		}
		for(i=0;i<C;i++) {
			for(j=0;j<D;j++) {
				scanf("%d",&F[i][j]);
			}
		}
		memset(memo,0,sizeof(memo));
		printf("%d\n",tansaku(0,0,0,W,X));
	}
	return 0;
}