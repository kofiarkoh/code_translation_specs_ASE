#include <stdio.h>

int main(void) {
	int a,b,judge;
	while(1) {
		judge=0;
		scanf("%d %d",&a,&b);
		if(a==0&&b==0) break;
		else
		for(;a<=b;a++) {
			if(a%400==0) {
				printf("%d\n",a);
				judge=1;
				continue;
			}
			else if(a%100==0) continue;
			else if(a%4==0) {
				printf("%d\n",a);
				judge=1;
			}
		}
		if(judge==0) printf("NA\n");
		printf("\n");
	}
	return 0;
}