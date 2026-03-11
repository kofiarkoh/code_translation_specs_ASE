#include<stdio.h>

int main(void) {

	int m, f, b, kari;
	scanf("%d %d %d", &m, &f, &b);
	if (m >= b) {
		printf("0\n");
	}
	else if(m + f < b){
		printf("NA\n");
	}
	else {
		kari = b - m;
		printf("%d\n", kari);
	}

	return 0;
}
