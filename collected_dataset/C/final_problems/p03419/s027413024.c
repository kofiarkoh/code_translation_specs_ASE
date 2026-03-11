#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <inttypes.h>

int main(void){
	long long int n,m;
	long long int ans;

	scanf("%lld %lld",&n,&m);
	if(n!=1&&n!=2){
		if(m!=1&&m!=2){
			ans=(n-2)*(m-2);
		}else if(m==1){
			ans=n-2;
		}else{
			ans=0;
		}
	}else if(n==1){
		if(m!=1&&m!=2){
			ans=m-2;
		}else if(m==1){
			ans=1;
		}else{
			ans=0;
		}
	}else{
		ans=0;
	}
	printf("%lld",ans);
	return 0;
}
