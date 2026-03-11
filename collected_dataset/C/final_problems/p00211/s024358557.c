#include<stdio.h>
#define N 10
typedef unsigned long long int INT;
INT GCD(INT a, INT b){
	if( b == 0 ) return a;
	else return GCD(b,a%b);
}
INT LCM(INT a, INT b){
	return (a/GCD(a,b))*b;
}
int main(){
	while(1){
		int i,n;
		INT d[N];
		INT v[N];
		INT Rates[N];
		INT lcmV = 1;
		INT gcdRates = 1;
		INT lcmRates = 1;

		scanf("%d",&n);
		if(0==n)break;

		for(i = 0; i < n; ++i){
			int td, tv;
			int gcd;
			scanf("%d%d",&td,&tv);
			gcd = GCD(td,tv);
			d[i] = (INT)td/gcd;
			v[i] = (INT)tv/gcd;
		}
		
		lcmV = LCM( v[0], v[1] );
		for(i = 2; i < n; ++i){
			lcmV = LCM( lcmV, v[i] );
		}
		for(i = 0; i < n; ++i){
			Rates[i] = (lcmV / v[i])*d[i];
		}
		gcdRates = GCD( Rates[0], Rates[1] );
		for(i = 2; i < n; ++i){
			gcdRates = GCD( gcdRates, Rates[i] );
		}
		for(i = 0; i < n; ++i){
			Rates[i] /= gcdRates;
		}
		lcmRates = LCM( Rates[0], Rates[1] );
		for(i = 2; i < n; ++i){
			lcmRates = LCM( lcmRates, Rates[i] );
		}
		for(i = 0; i < n; ++i){
			printf("%u\n", (unsigned int)(lcmRates / Rates[i]));
		}
	}
	return 0;
}