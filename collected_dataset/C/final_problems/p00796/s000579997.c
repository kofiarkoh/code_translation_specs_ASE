#include <stdio.h>
#include <math.h>

#define REP(i,n,m) for(i=n;i<m;i++)
#define rep(i,n) REP(i,0,n)

#define EPS (1e-5)

typedef struct{
	double x,y,z;
}P;

int main(void){
	int i,j,k;
	int n,m;
	double a,b,c;
	P p[30];

	scanf("%d",&n);
	while(n--){
		scanf("%lf%lf%lf%d",&a,&b,&c,&m);
		rep(i,m) scanf("%lf%lf%lf",&p[i].x,&p[i].y,&p[i].z);

		rep(i,m){
			rep(j,m){
				rep(k,m){
					if(i == j || i == k || j == k) continue;

					double ta = sqrt( pow(p[j].x - p[i].x, 2) +
														pow(p[j].y - p[i].y, 2) + 
														pow(p[j].z - p[i].z, 2));
					double tb = sqrt( pow(p[k].x - p[j].x, 2) +
														pow(p[k].y - p[j].y, 2) + 
														pow(p[k].z - p[j].z, 2));
					double tc = sqrt( pow(p[i].x - p[k].x, 2) +
														pow(p[i].y - p[k].y, 2) + 
														pow(p[i].z - p[k].z, 2));

					if(a / b - ta / tb < EPS &&
						 b / c - tb / tc < EPS &&
						 c / a - tc / ta < EPS)
						printf("%d %d %d\n",k+1,i+1,j+1);
				}
			}
		}
	}

	return 0;
}