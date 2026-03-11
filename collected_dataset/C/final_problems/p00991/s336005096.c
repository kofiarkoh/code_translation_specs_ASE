#include <stdio.h>

/* this is prime */
#define MOD_BY 100000007

int getInv(int x) {
	int nowzyou=MOD_BY-2;
	long long now=x;
	long long result=1;
	while(nowzyou>0) {
		if(nowzyou & 1)result=(result*now)%MOD_BY;
		now=(now*now)%MOD_BY;
		nowzyou>>=1;
	}
	return (int)result;
}

int nCr(int n,int r) {
	long long result=1;
	int i;
	for(i=n;i>n-r;i--)result=(result*i)%MOD_BY;
	for(i=1;i<=r;i++)result=(result*getInv(i))%MOD_BY;
	return (int)result;
}

int main(void) {
	int w,h,sx,sy,dx,dy;
	int x,y,x1,y1,x2,y2,now;
	int result;
	scanf("%d%d%d%d%d%d",&w,&h,&sx,&sy,&dx,&dy);
	x=(w-dx)+(sx-0);
	now=(w-sx)+(dx-0);if(now<x)x=now;
	x1=x;
	now=dx-sx;if(now<0)now=-now;if(now<x)x=now;
	x2=now;
	y=(h-dy)+(sy-0);
	now=(h-sy)+(dy-0);if(now<y)y=now;
	y1=y;
	now=dy-sy;if(now<0)now=-now;if(now<y)y=now;
	y2=now;
	result=nCr(x+y,x);
	if(x1==x2)result=(result*2)%MOD_BY;
	if(y1==y2)result=(result*2)%MOD_BY;
	printf("%d\n",result);
	return 0;
}