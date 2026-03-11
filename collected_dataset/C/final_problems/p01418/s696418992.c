#include<stdio.h>
#include<math.h>
int K,R,L,tmp=0;
double P,E,T;

double dfs(double r,double l,int c)
{
	double h=(r+l)/2,t=h-T;
	if(c==K)return(-E<=t&&t<=E)?1.0:0.0;
	if(E<T-l||E<r-T)return 0.0;
	if(E>l-r)return 1.0;
	if(h>=T) return dfs(h,l,c+1)*P+dfs(r,h,c+1)*(1.0-P);
	else return dfs(h,l,c+1)*(1.0-P)+dfs(r,h,c+1)*P;
}

int main()
{
	scanf("%d%d%d%lf%lf%lf",&K,&R,&L,&P,&E,&T);
	printf("%lf\n",dfs((double)R,(double)L,0));
	return 0;
}