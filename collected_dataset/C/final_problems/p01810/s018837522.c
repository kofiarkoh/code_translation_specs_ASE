#include<stdio.h>
#define P(type,x) fprintf(stdout,"%"#type"\n",x)
#define S(type,x) fscanf(stdin,"%"#type,&x)

int main() {
	int n, k, i;
	unsigned long long a = 0;
	S(d, n), S(d, k);
	for (i = 0; i < n - 1; i++) a += a / (k - 1) + 1;
	P(lld, a);
	return 0;
}