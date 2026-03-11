#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define EPS 1e-9

typedef struct {
	double x,y;
} xy_t;

double eq(double a, double b) {
	return fabs(a-b)<EPS;
}

double lt(double a, double b) {
	return a+EPS<b;
}

double leq(double a, double b) {
	return a<b+EPS;
}

double gt(double a, double b) {
	return a>b+EPS;
}

double geq(double a, double b) {
	return a+EPS>b;
}

int inrange(double r1, double r2, double c) {
	if(lt(r1,r2)) {
		return leq(r1,c) && leq(c,r2);
	} else if(gt(r1,r2)) {
		return leq(r2,c) && leq(c,r1);
	} else {
		return eq(r1,c) && eq(r2,c);
	}
}

xy_t kouten(int *e, xy_t s1, xy_t t1, xy_t s2, xy_t t2) {
	/* (x,y) = (x1,y1)+t(x2-x1,y2-y1)
	 * t = (x-x1)/(x2-x1) = (y-y1)/(y2-y1)
	 * (x-x1)*(y2-y1) = (y-y1)*(x2-x1)
	 * (y2-y1)*x + (x1-x2)*y = x1*y2 - x2*y1
	 */
	xy_t ret={0,0};
	double a = t1.y-s1.y;
	double b = s1.x-t1.x;
	double c = t2.y-s2.y;
	double d = s2.x-t2.x;
	double v1 = s1.x*t1.y - t1.x*s1.y;
	double v2 = s2.x*t2.y - t2.x*s2.y;
	double delta = a*d-b*c;
	if(eq(delta,0)) {
		if(e!=NULL)*e=0;
		return ret;
	}
	ret.x=(d*v1-b*v2)/delta;
	ret.y=(-c*v1+a*v2)/delta;
	if(e!=NULL) {
		*e=inrange(s1.x,t1.x,ret.x) && inrange(s1.y,t1.y,ret.y) &&
			inrange(s2.x,t2.x,ret.x) && inrange(s2.y,t2.y,ret.y);
	}
	return ret;
}

typedef struct {
	xy_t zahyo;
	int is_kouka;
} kouten_t;

int cmp_x(const void *x, const void *y) {
	double a = ((const kouten_t*)x)->zahyo.x;
	double b = ((const kouten_t*)y)->zahyo.x;
	if(gt(a,b))return 1;
	if(lt(a,b))return -1;
	return 0;
}

int cmp_y(const void *x, const void *y) {
	double a = ((const kouten_t*)x)->zahyo.y;
	double b = ((const kouten_t*)y)->zahyo.y;
	if(gt(a,b))return 1;
	if(lt(a,b))return -1;
	return 0;
}

int solve(void) {
	xy_t a,b;
	int n;
	int i;
	kouten_t kouten_list[110];
	int kouten_num=0;
	int ans;
	if(scanf("%lf%lf%lf%lf",&a.x,&a.y,&b.x,&b.y)!=4)return 0;
	if(scanf("%d",&n)!=1)return 0;
	for(i=0;i<n;i++) {
		xy_t s,t;
		int o,l;
		xy_t k;
		int e=0;
		if(scanf("%lf%lf%lf%lf%d%d",&s.x,&s.y,&t.x,&t.y,&o,&l)!=6)return 0;
		k=kouten(&e,a,b,s,t);
		if(e) {
			kouten_list[kouten_num].zahyo=k;
			kouten_list[kouten_num].is_kouka = (1-o)^l;
			kouten_num++;
		}
	}
	qsort(kouten_list,kouten_num,sizeof(kouten_list[0]),
		  eq(kouten_list[0].zahyo.x,kouten_list[kouten_num-1].zahyo.x)?cmp_y:cmp_x);
	ans=0;
	for(i=1;i<kouten_num;i++) {
		if(kouten_list[i].is_kouka!=kouten_list[i-1].is_kouka)ans++;
	}
	printf("%d\n",ans);
	return 1;
}

int main(void) {
	int n,i;
	if(scanf("%d",&n)!=1)return 1;
	for(i=0;i<n;i++) {
		if(!solve())return 1;
	}
	return 0;
}