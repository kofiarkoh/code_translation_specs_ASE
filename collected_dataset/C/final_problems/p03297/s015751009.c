#include <stdio.h>

int gcd(long long int a, long long int b){
	return a%b==0?b:gcd(b,a%b);
}

int main(void){
	int t;
	scanf("%d",&t);
	for(int i=0;i<t;i++){
		long long int a,b,c,d;
		scanf("%lld%lld%lld%lld",&a,&b,&c,&d);
		if(b>a || b > d){
			printf("No\n");
			continue;
		}
		else if(c >= b){
			printf("Yes\n");
			continue;
		}
		//在庫の最大値
		long long int g = gcd(d,b);
		if(b-g+a%g > c){
			printf("No\n");
		}
		else{
			printf("Yes\n");
		}
	}
	return 0;
}
