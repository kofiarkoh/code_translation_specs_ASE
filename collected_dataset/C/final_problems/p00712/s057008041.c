#include <stdio.h>

long long gcd(long long a,long long b) {
	return b==0?a:gcd(b,a%b);
}

int p,q,a,n;

long long tansaku(long long nowp,long long nowq,int nowa,int n,int start) {
	long long result=0;
	int i;
	if(n<=0)return 0;
	if(n==1) {
		long long pp,qq,gg;
		pp=p*nowq-nowp*q;
		qq=q*nowq;
		gg=gcd(pp,qq);
		if(gg>1){pp/=gg;qq/=gg;}
		return (pp==1 && qq<=start && nowa*qq<=a)?1:0;
	}
	/* if((p*nowq-nowp*q)/q*nowq>n)return 0; */
	if((p-n*q)*nowq-nowp*q>0)return 0;
	i=start;
	if((long long)nowa*i>a)i=a/nowa;
	for(;i>0;i--) {
		long long pp,qq,gg;
		pp=nowp*i+nowq;
		qq=nowq*i;
		gg=gcd(pp,qq);
		if(gg>1){pp/=gg;qq/=gg;}
		if(pp*q>p*qq)break;
		else if(pp*q==p*qq){result++;break;}
		else result+=tansaku(pp,qq,nowa*i,n-1,i);
	}
	return result;
}

int main(void) {
	while(scanf("%d%d%d%d",&p,&q,&a,&n)==4 && (p|q|a|n)!=0) {
		int g=gcd(p,q);
		p/=g;q/=g;
		printf("%lld\n",tansaku(0,1,1,n,a));
	}
	return 0;
}