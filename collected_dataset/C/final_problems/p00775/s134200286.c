#include <stdio.h>
#include <math.h>

#define max(a, b) ((a)>(b)?(a):(b))
#define min(a, b) ((a)<(b)?(a):(b))

int skyline[41];

double calc(int x, int y, int r){
	return y+r-sqrt((double)(r*r-x*x));
}

int main(void){
	int n,r;
	while(scanf("%d %d", &r, &n),r||n){
		int i,j,xl,xr,h;
		for(i=0;i<41;i++) skyline[i]=0;

		for(i=0;i<n;i++){
			scanf("%d %d %d", &xl, &xr, &h);
			for(j=xl;j<xr;j++){
				if(skyline[j+20]<h)skyline[j+20]=h;
			}
		}
		double t=calc(0,min(skyline[19],skyline[20]),r);
		double tt;
		int tx,ty;
		tx=0;
		ty=min(skyline[19],skyline[20]);
		for(j=0;j<r-1;j++){
			if(skyline[j+20]>skyline[j+1+20]){
				tt=calc(j+1,skyline[j+1+20],r);
				if(tt<t){
					tx=j+1;
					ty=skyline[j+21];
				}
				t=min(t,tt);
			}
		}
		for(j=-1;j>0-r;j--){
			if(skyline[j+20]>skyline[j-1+20]){
				tt=calc(j,skyline[j-1+20],r);
				if(tt<t){
					tx=j;
					ty=skyline[j+19];
				}
				t=min(t,tt);
			}
		}
		printf("%0.4f\n",t);
	}

	return 0;
}