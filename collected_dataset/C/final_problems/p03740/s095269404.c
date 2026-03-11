#include <stdio.h>

typedef long long ll;

ll max(ll a, ll b){
	if(a>b){return a;}
	else{return b;}
}

int main(){

	ll X,Y;

	scanf("%lld %lld",&X,&Y);

	if(max(X-Y,Y-X)<=1ll){
	printf("Brown\n");
	}
	else{
	printf("Alice\n");
	}

	return 0;
}