#include <stdio.h>

#define EPS (1e-7)

double calc_square(int x1,int y1,int x2,int y2,int x3,int y3) {
	int vx1,vy1,vx2,vy2;
	double result;
	vx1=x2-x1;vy1=y2-y1;
	vx2=x3-x1;vy2=y3-y1;
	result=vx1*vy2-vx2*vy1;
	if(result<0)result=-result;
	return result/2.0;
}

int isCross(int x1,int y1,int x2,int y2,
		int x3,int y3,int x4,int y4) {
	long long b12x,b12y,b34x,b34y;
	long long b24x,b24y;
	long long b13x,b13y,b14x,b14y,b32x,b32y;
	long long g1213,g1214,g3431,g3432;
	long long n1213,n1214,n3431,n3432;
	long long n2123,n2124,n4341,n4342;

	b12x=x2-x1;b12y=y2-y1;
	b34x=x4-x3;b34y=y4-y3;
	b13x=x3-x1;b13y=y3-y1;
	b14x=x4-x1;b14y=y4-y1;
	b32x=x2-x3;b32y=y2-y3;
	b24x=x4-x2;b24y=y4-y2;

	g1213=b12x*b13y-b13x*b12y;
	g1214=b12x*b14y-b14x*b12y;
	g3431=b34x*(-b13y)-(-b13x)*b34y;
	g3432=b34x*b32y-b32x*b34y;

	n1213=b12x*b13x+b12y*b13y;
	n1214=b12x*b14x+b12y*b14y;
	n3431=b34x*(-b13x)+b34y*(-b13y);
	n3432=b34x*b32x+b34y*b32y;
	n2123=(-b12x)*(-b32x)+(-b12y)*(-b32y);
	n2124=(-b12x)*b24x+(-b12y)*b24y;
	n4341=(-b34x)*(-b14x)+(-b34y)*(-b14y);
	n4342=(-b34x)*(-b24x)+(-b34y)*(-b24y);

	if(
		((g1213<0 && g1214>0) || (g1213>0 && g1214<0)) &&
		((g3431<0 && g3432>0) || (g3431>0 && g3432<0))
	) return 1;
	if(
		(g1213==0 && n1213>=0 && n2123>=0) ||
		(g1214==0 && n1214>=0 && n2124>=0) ||
		(g3431==0 && n3431>=0 && n4341>=0) ||
		(g3432==0 && n3432>=0 && n4342>=0)
	) return 1;

	return 0;
}

int isInTriangle(int x,int y,int x1,int y1,int x2,int y2,int x3,int y3) {
	int vx1,vy1,vx2,vy2;
	int tvx,tvy;
	int delta;
	double s,t;
	vx1=x2-x1;vy1=y2-y1;
	vx2=x3-x1;vy2=y3-y1;
	tvx=x-x1;tvy=y-y1;
	delta=x1*y2-x2*y1;
	if(delta==0)return 0;
	s=(double)(y2*x-x2*y)/delta;
	t=(double)(-y1*x+x1*y)/delta;
	return s+EPS>0 && t+EPS>0 && s+t<1.0+EPS;
}

int main(void) {
	int case_count;
	int n,i;
	int x[50],y[50];
	int prev[50],next[50];
	int triangles[50][3];
	int now;
	int choten_rest;
	int triangle_count;
	double square;
	for(case_count=1;scanf("%d",&n)==1 && n>0;case_count++) {
		for(i=0;i<n;i++) {
			scanf("%d%d",&x[i],&y[i]);
			prev[i]=(i==0?n-1:i-1);
			next[i]=(i==n-1?0:i+1);
		}
		choten_rest=n;
		triangle_count=0;
		for(now=0;choten_rest>3;now=next[now]) {
			/* hekonnde inai */
			if((x[prev[now]]-x[now])*(y[next[now]]-y[now])-(x[next[now]]-x[now])*(y[prev[now]]-y[now])>0) {
				/* tugiga nakani haitte inai */
				if(!isInTriangle(x[prev[prev[now]]],y[prev[prev[now]]],
						x[now],y[now],x[prev[now]],y[prev[now]],x[next[now]],y[next[now]]) &&
					!isInTriangle(x[next[next[now]]],y[next[next[now]]],
						x[now],y[now],x[prev[now]],y[prev[now]],x[next[now]],y[next[now]])) {
					/* sennga kasanatte inai */
					int ok=1;
					for(i=next[next[now]];i!=prev[prev[now]];i=next[i]) {
						if(isCross(x[prev[now]],y[prev[now]],x[next[now]],y[next[now]],
							x[i],y[i],x[next[i]],y[next[i]])){ok=0;break;}
					}
					if(ok) {
						triangles[triangle_count][0]=now;
						triangles[triangle_count][1]=prev[now];
						triangles[triangle_count][2]=next[now];
						next[prev[now]]=next[now];
						prev[next[now]]=prev[now];
						choten_rest--;
						triangle_count++;
					}
				}
			}
		}
		triangles[triangle_count][0]=now;
		triangles[triangle_count][1]=prev[now];
		triangles[triangle_count][2]=next[now];
		triangle_count++;
		square=0;
		for(i=0;i<triangle_count;i++) {
			square+=calc_square(
				x[triangles[i][0]],y[triangles[i][0]],
				x[triangles[i][1]],y[triangles[i][1]],
				x[triangles[i][2]],y[triangles[i][2]]
			);
		}
		printf("%d %.1f\n",case_count,square);
	}
	return 0;
}