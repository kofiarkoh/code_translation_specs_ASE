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
		INT Ans[N];
		INT lcmD = 1;
		INT gcdAns = 1;

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
		
		lcmD = LCM( d[0], d[1] );
		for(i = 2; i < n; ++i){
			lcmD = LCM( lcmD, d[i] );
		}
		for(i = 0; i < n; ++i){
			Ans[i] = (lcmD / d[i])*v[i];
		}
		gcdAns = GCD( Ans[0], Ans[1] );
		for(i = 2; i < n; ++i){
			gcdAns = GCD( gcdAns, Ans[i] );
		}
		for(i = 0; i < n; ++i){
			Ans[i] /= gcdAns;
		}
		for(i = 0; i < n; ++i){
			printf("%u\n", (unsigned int)(Ans[i]));
		}
	}
	return 0;
}