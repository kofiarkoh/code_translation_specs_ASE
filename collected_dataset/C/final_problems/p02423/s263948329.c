#include<stdio.h>
#include<math.h>

int main() {
	unsigned int x;
	int a[33] = {0},b[33] = { 0 }, c[33] = { 0 }, i;

	scanf("%d", &x);

	for (i =0; x>0; i++) {
		a[i] = x % 2;
		x = x / 2;
	}

	for (i = 31; i >= 0; i--)printf("%d",a[i]);
	printf("\n");


	for(i = 31; i >= 0; i--) {
		if (a[i]==0)printf("1");
		else printf("0");
	}
	printf("\n");


	for (i = 31; i >= 0; i--) {
		b[i+1] = a[i];
	}
	for (i = 31; i >= 0; i--)printf("%d", b[i]);
	
	printf("\n");


	for (i = 31; i >= 0; i--) {
		c[i] = a[i+1];
	}
	for (i = 31; i >= 0; i--)printf("%d",c[i]);
	printf("\n");


	return 0;
}
