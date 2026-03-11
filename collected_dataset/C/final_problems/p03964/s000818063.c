#include<stdio.h>
#define FOR(i,a,b) for(int i=a;i<b;i++)
#define ll long long
int main(){
	int n,t,a;
	ll T=1,A=1;
	scanf("%d",&n);
	FOR(i,0,n){
		scanf("%d %d",&t,&a);
		int max = (T+t-1)/t > (A+a-1)/a ?  (T+t-1)/t : (A+a-1)/a;
		T = t * max;
		A = a * max;
	}
	printf("%lld\n",T+A);
}