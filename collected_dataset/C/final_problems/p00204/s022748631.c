#include <stdio.h>
#include <math.h>

#define EPS (1e-8)

typedef struct {
	double x,y;
	double angle;
	int r;
	int v;
	int arrive;
} ufo_t;

int N;
ufo_t ufo[100];
int R;

double get_dist(double mx,double my,double x,double y) {
	return fabs(my*x-mx*y)/sqrt(my*my+mx*mx);
}

int main(void) {
	int i;
	int target;
	double target_dist;
	int count;
	while(1) {
		scanf("%d%d",&R,&N);
		if(R==0 && N==0)break;
		for(i=0;i<N;i++) {
			scanf("%lf%lf%d%d",
				&ufo[i].x,&ufo[i].y,&ufo[i].r,&ufo[i].v);
			ufo[i].arrive=1;
			ufo[i].angle=atan2(ufo[i].y,ufo[i].x);
		}
		target=0;
		while(target>=0) {
			for(i=0;i<N;i++) {
				if(R+EPS<sqrt(ufo[i].x*ufo[i].x+ufo[i].y*ufo[i].y)) {
					double prev_x,prev_y;
					prev_x=ufo[i].x;
					prev_y=ufo[i].y;
					ufo[i].x-=ufo[i].v*cos(ufo[i].angle);
					ufo[i].y-=ufo[i].v*sin(ufo[i].angle);
					if(prev_x*ufo[i].x<0)ufo[i].x=0;
					if(prev_y*ufo[i].y<0)ufo[i].y=0;
				}
			}
			target=-1;
			target_dist=1000*1000;
			for(i=0;i<N;i++) {
				double dist;
				if(!ufo[i].arrive)continue;
				dist=sqrt(ufo[i].x*ufo[i].x+ufo[i].y*ufo[i].y);
				if(dist<target_dist+EPS && R+EPS<dist) {
					target_dist=dist;
					target=i;
				}
			}
			if(target<0)break;
			for(i=0;i<N;i++) {
				if(sqrt(ufo[i].x*ufo[i].x+
					ufo[i].y*ufo[i].y)<R+EPS)continue;
				if(get_dist(ufo[target].x,ufo[target].y,
						ufo[i].x,ufo[i].y)<ufo[i].r+EPS &&
						ufo[target].x*ufo[i].x>=0 &&
						ufo[target].y*ufo[i].y>=0) {
					ufo[i].arrive=0;
				}
			}
		}
		count=0;
		for(i=0;i<N;i++) {
			if(ufo[i].arrive)count++;
		}
		printf("%d\n",count);
	}
	return 0;
}