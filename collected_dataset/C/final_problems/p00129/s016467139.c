#include <stdio.h>
#include <math.h>

#define EPS (1e-7)

typedef struct {
	int x,y,r;
} wall_t;

typedef struct {
	int tx,ty,sx,sy;
} query_t;

int hitcheck_circle(wall_t w,query_t q) {
	int in=0;
	int a,b,c;
	int minx,miny,maxx,maxy,temp;
	/* sieve */
	minx=q.tx;maxx=q.sx;
	if(minx>maxx){temp=minx;minx=maxx;maxx=temp;}
	miny=q.ty;maxy=q.sy;
	if(miny>maxy){temp=miny;miny=maxy;maxy=temp;}
	if(w.x+w.r<minx || maxx<w.x-w.r)return 0;
	if(w.y+w.r<miny || maxy<w.y-w.r)return 0;
	/* check if dist<=r */
	if((q.tx-w.x)*(q.tx-w.x)+(q.ty-w.y)*(q.ty-w.y)<=w.r*w.r)in++;
	if((q.sx-w.x)*(q.sx-w.x)+(q.sy-w.y)*(q.sy-w.y)<=w.r*w.r)in++;
	if(in==1)return 1;
	if(in==2)return 0;
	a=q.ty-q.sy;
	b=q.sx-q.tx;
	c=(q.tx-q.sx)*q.ty-(q.ty-q.sy)*q.tx;
	if((a*w.x+b*w.y+c)*(a*w.x+b*w.y+c)<=w.r*w.r*(a*a+b*b)) {
		/* check if really hit */
		c=-c; /* convert a*x+b*y+c=0 to a*x+b*y=c */
		if(b==0) {
			double yy=w.r*w.r-((double)c/a-w.x)*((double)c/a-w.x);
			double y1,y2;
			y1=sqrt(yy);y2=-y1;
			y1+=w.y;y2+=w.y;
			return (miny<y1+EPS && y1<maxy+EPS) || (miny<y2+EPS && y2<maxy+EPS);
		} else {
			double cb=(double)c/b;
			double ab=(double)a/b;
			double x2k,xk,tk;
			double x1,x2;
			x2k=1.0+ab*ab;
			xk=-2*w.x+2.0*(w.y-cb)*ab;
			tk=w.x*w.x+(w.y-cb)*(w.y-cb)-w.r*w.r;
			x1=(-xk+sqrt(xk*xk-4.0*x2k*tk))/(2.0*x2k);
			x2=(-xk-sqrt(xk*xk-4.0*x2k*tk))/(2.0*x2k);
			return (minx<x1+EPS && x1<maxx+EPS) || (minx<x2+EPS && x2<maxx+EPS);
		}
	}
	return 0;
}
int main(void) {
	int n;
	wall_t w[100];
	int m;
	int i;
	while(scanf("%d",&n)==1 && n>0) {
		for(i=0;i<n;i++)scanf("%d%d%d",&w[i].x,&w[i].y,&w[i].r);
		scanf("%d",&m);
		for(i=0;i<m;i++) {
			query_t q;
			int j;
			int hidden;
			scanf("%d%d%d%d",&q.tx,&q.ty,&q.sx,&q.sy);
			hidden=0;
			if(q.tx!=q.sx || q.ty!=q.sy) {
				for(j=0;j<n;j++) {
					if(hitcheck_circle(w[j],q)) {
						hidden=1;
						break;
					}
				}
			}
			puts(hidden?"Safe":"Danger");
		}
	}
	return 0;
}