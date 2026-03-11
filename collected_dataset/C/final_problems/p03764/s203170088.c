#include <stdio.h>
int main(void){
	long long p=1000000007;
	long long n,m;
	scanf("%lld%lld",&n,&m);
	long long x[n];
	long long y[n];
	for(int i=0;i<n;i++){
		scanf("%lld",&x[i]);
	}
	for(int i=0;i<m;i++){
		scanf("%lld",&y[i]);
	}
	long long s=0;
	long long e,f;
	for(int i=0;i<n-1;i++){
		e=(i+1)*(n-i-1)%p;
		s+=e*(x[i+1]-x[i])%p;
		if(s>p)s-=p;
	}
	s=(s+p)%p;
	long long ans=0;
	for(int i=0;i<m-1;i++){
		e=(i+1)*(m-i-1)%p;
		f=e*(y[i+1]-y[i])%p;
		ans+=f*s%p;
		if(ans>p)ans-=p;
	}
	printf("%lld\n",(ans+p)%p);
}