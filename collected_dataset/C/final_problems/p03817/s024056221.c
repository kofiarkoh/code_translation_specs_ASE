#include <stdio.h>

int main (void){

	long long int x;
	scanf("%lld", &x);

	long long int ans = 0;
	long long int syo = 0, amari = 0;

	syo = x / 11;
	amari = x % 11;		
	ans = syo * 2 + 1;

	if(amari == 0){
		printf("%lld\n", ans - 1);
		return 0;
	}else if(0 < amari && amari < 7){
		printf("%lld\n", ans);
		return 0;
	}else{
		printf("%lld\n", ans + 1);
		return 0;
	}
	 

}