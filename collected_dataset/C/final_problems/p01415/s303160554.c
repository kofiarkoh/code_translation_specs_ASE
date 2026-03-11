#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define EPS 1e-9

int d[300];
int main(){
	int n,k,t,u,v,l;
	scanf("%d%d%d%d%d%d",&n,&k,&t,&u,&v,&l);
	rep(i,0,n)scanf("%d",d+i);
	double ans=0;
	int crr=0,cnt=0;
	double time=0;
	rep(i,0,l){
		if(d[crr]==i)cnt++,crr++;
		if(fabs(time)<EPS&&cnt>0)time=t,cnt--;
		if(cnt>k)cnt=k,time=t;
		ans+=time>EPS?1./v:1./u;
		if(time>EPS)time-=1./v;
	}
	printf("%.9f\n",ans);
}
