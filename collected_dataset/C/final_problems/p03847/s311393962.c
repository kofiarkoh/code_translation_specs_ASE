#include <stdio.h>
#define size 100
#define p (int)(1e9 + 7)

int main(){
	int i;
	long long int K[size], L1[size], L2[size], L3[size];
	long long int n;
	scanf("%lld", &n);

	for(i = 0; n != 0; i++){
		K[i] = n;
		n = (n-1)/2;
	}
	K[i]  = 1;
	L1[i] = 1;
	L2[i] = 1;
	L3[i] = 0;
	i--;

	for(; i >= 0; i--){
		if(K[i] % 2 == 0){
			K[i] = (3*K[i + 1] + L1[i + 1])%p;
			L1[i] = L1[i + 1];
			L2[i] = (L1[i + 1] + L2[i + 1])%p;
			L3[i] = L2[i + 1];
		}
		else{
			K[i] = (3*K[i + 1] - L2[i + 1] + p)%p;
			L1[i] = (L1[i + 1] + L2[i + 1])%p;
			L2[i] = L2[i + 1];
			L3[i] = (L2[i + 1] + L3[i + 1])%p;
		}
	}
	printf("%lld\n", K[0]);
	return 0;
}