#include <stdio.h>
#define M 1000000007

int main(){
	long a, b, x, n;
	scanf("%ld %ld %ld", &a, &b, &x);
	if(x - b >=0){
    	printf("%ld\n", (x + ( ( (x - b)/(a - b) )%M) * (b%M) ) %M );
	}else{
	    printf("%ld\n", x % M);
    }
}
