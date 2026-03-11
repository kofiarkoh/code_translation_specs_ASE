#include <stdio.h>
int main(void){
	long long  n,m,ans;
	ans = 0;
	scanf("%lld%lld",&n,&m);
	if( (n-2)>=0 || (m-2)>=0){
		ans = (n-2)*(m-2);
	}
	if( n==1){
		ans = m-2;
	}else if( n!= 1 &&m==1){
		ans = n-2;
	}
	if( ans == -1){
		ans = 1;
	}else{
	}
	printf("%lld",ans);
}
