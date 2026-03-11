#include<stdio.h>
#include<stdlib.h>
double res=100000000.0,t;
int T,D,t1,t2,d1,d2,p1,p2;
int main()
{
	scanf("%d%d%d%d%d%d",&T,&D,&t1,&t2,&d1,&d2);
	for(p1=0;p1<=D;p1+=d1)
		for(p2=0;p2<=D-p1;p2+=d2)
		{
			if(p1+p2==0)continue;
			t=(1.0*(t1*p1)+(t2*p2))/(p1+p2);
			t-=T;
			if(t<0)t*=-1;
			res=(res-t>0)?t:res;
		}
	printf("%.10f\n",res);
	return 0;
}