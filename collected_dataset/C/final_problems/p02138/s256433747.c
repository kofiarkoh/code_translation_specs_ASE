#include<stdio.h>
#define min(p,q)((p)<(q)?(p):(q))

int main(){
	long long X,Y;
	scanf("%lld%lld",&X,&Y);
	int ans1=0;
	//aを滅ぼす場合
	long long a=X*2,b1=0,b2=Y;
	while(1){
		long long t=(a+1)/2;
		long long tt=min(b2,t);
		b2-=tt;
		b1+=tt;
		b1-=min(b1,t-tt);
		if(b1+b2==0)break;
		ans1++;
		a-=b1+b2;
		if(a<=0)break;
		ans1++;
	}

	int ans2=0;
	//bを滅ぼす場合
	long long b=Y*2,a1=0,a2=X;
	while(1){
		b-=a1+a2;
		if(b<=0)break;
		ans2++;
		long long t=(b+1)/2;
		long long tt=min(a2,t);
		a2-=tt;
		a1+=tt;
		a1-=min(a1,t-tt);
		if(a1+a2==0)break;
		ans2++;
	}
	printf("%d\n",min(ans1,ans2));
}
