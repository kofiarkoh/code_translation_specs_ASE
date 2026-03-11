#include <stdio.h>

int main(){
	int  q, h, s, d;
	int  n;
	long int value = 0;

	scanf("%d %d %d %d", &q, &h, &s, &d);
	scanf("%d", &n);

	if(2*q < h) h = 2*q;
	if(2*h < s) s = 2*h;
	if(2*s < d) d = 2*s;

	value += (long)d * (long)(n / 2);
	if(n % 2)
		value += s;

	printf("%ld\n", value);

}