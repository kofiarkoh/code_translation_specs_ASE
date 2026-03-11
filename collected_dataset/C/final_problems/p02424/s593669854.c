#include<stdio.h>

void di2bi(unsigned long x, int* b){
	int i;
	
	for(i=32-1; i>=0; i--){
		b[i] = (x>>i)&1;
	}
}



int main(){
	int i;
	unsigned long a, b;
	int bi_a[32], bi_b[32];
	int calc[32];
	
	scanf("%ld %ld", &a, &b);
	
	di2bi(a, bi_a);
	di2bi(b, bi_b);
	
	/*AND*/
	for(i=32-1; i>=0; i--)	calc[i] = bi_a[i]&bi_b[i];
	
	for(i=32-1; i>=0; i--)	printf("%d", calc[i]);
	printf("\n");
	
	/*OR*/
	for(i=32-1; i>=0; i--)	calc[i] = bi_a[i]|bi_b[i];
	
	for(i=32-1; i>=0; i--)	printf("%d", calc[i]);
	printf("\n");
	
	/*XOR*/
	for(i=32-1; i>=0; i--)	calc[i] = bi_a[i]^bi_b[i];
	
	for(i=32-1; i>=0; i--)	printf("%d", calc[i]);
	printf("\n");
	
	return 0;
}
