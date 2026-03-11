#include <stdio.h>
#include <stdlib.h>

typedef struct {
	double x,y;
} xy_t;

int qsort_comp(const void* x,const void* y) {
	const xy_t* a=(const xy_t*)x;
	const xy_t* b=(const xy_t*)y;
	if((a->x)>(b->x))return 1;
	if((a->x)<(b->x))return -1;
	if((a->y)>(b->y))return 1;
	if((a->y)<(b->y))return -1;
	return 0;
}

int zahyou_num;
xy_t zahyou[100];

int th_up_n;
int th_down_n;
xy_t th_up[100];
xy_t th_down[100];

int main(void) {
	int i;
	while(1) {
		scanf("%d",&zahyou_num);
		if(zahyou_num==0)break;
		for(i=0;i<zahyou_num;i++) {
			scanf("%lf,%lf",&zahyou[i].x,&zahyou[i].y);
		}
		qsort(zahyou,zahyou_num,sizeof(xy_t),qsort_comp);
		//sita no totu-ho
		th_down_n=1;
		th_down[0].x=zahyou[0].x;
		th_down[0].y=zahyou[0].y;
		for(i=1;i<zahyou_num;i++) {
			//if(zahyou[i].y<th_down[th_down_n-1].y) {
				while(th_down_n>1) {
					double k1,k2;
					if(zahyou[i].x==th_down[th_down_n-1].x) {
						//INF
						if(zahyou[i].y<th_down[th_down_n-1].y) {
							k1=-1e+200;
						} else if(zahyou[i].y>th_down[th_down_n-1].y) {
							k1=1e+200;
						} else k1=0;
					} else {
						k1=(zahyou[i].y-th_down[th_down_n-1].y)/
						   (zahyou[i].x-th_down[th_down_n-1].x);
					}
					if(th_down[th_down_n-1].x==th_down[th_down_n-2].x) {
						//INF
						if(th_down[th_down_n-1].y<th_down[th_down_n-2].y) {
							k2=-1e+200;
						} else if(th_down[th_down_n-1].y>th_down[th_down_n-2].y) {
							k2=1e+200;
						} else k2=0;
					} else {
						k2=(th_down[th_down_n-1].y-th_down[th_down_n-2].y)/
						   (th_down[th_down_n-1].x-th_down[th_down_n-2].x);
					}
					if(k1>=k2)break;
					th_down_n--;
				}
			//}
			th_down[th_down_n].x=zahyou[i].x;
			th_down[th_down_n].y=zahyou[i].y;
			th_down_n++;
		}
		//ue no totu-ho
		th_up_n=1;
		th_up[0].x=zahyou[zahyou_num-1].x;
		th_up[0].y=zahyou[zahyou_num-1].y;
		for(i=zahyou_num-2;i>=0;i--) {
			//if(zahyou[i].y>th_up[th_up_n-1].y) {
				while(th_up_n>1) {
					double k1,k2;
					if(zahyou[i].x==th_up[th_up_n-1].x) {
						//INF
						if(zahyou[i].y<th_up[th_up_n-1].y) {
							k1=1e+200;
						} else if(zahyou[i].y>th_up[th_up_n-1].y) {
							k1=-1e+200;
						} else k1=0;
					} else {
						k1=(zahyou[i].y-th_up[th_up_n-1].y)/
						   (zahyou[i].x-th_up[th_up_n-1].x);
					}
					if(th_up[th_up_n-1].x==th_up[th_up_n-2].x) {
						//INF
						if(th_up[th_up_n-1].y<th_up[th_up_n-2].y) {
							k2=1e+200;
						} else if(th_up[th_up_n-1].y>th_up[th_up_n-2].y) {
							k2=-1e+200;
						} else k2=0;
					} else {
						k2=(th_up[th_up_n-1].y-th_up[th_up_n-2].y)/
						   (th_up[th_up_n-1].x-th_up[th_up_n-2].x);
					}
					if(k1>=k2)break;
					th_up_n--;
				}
			//}
			th_up[th_up_n].x=zahyou[i].x;
			th_up[th_up_n].y=zahyou[i].y;
			th_up_n++;
		}
		printf("%d\n",zahyou_num-(th_down_n+th_up_n-2));
	}
	return 0;
}