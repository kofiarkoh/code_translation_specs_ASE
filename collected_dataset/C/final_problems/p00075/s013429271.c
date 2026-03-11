#include <stdio.h>

int main(void) {
	int num;
	double wei, hei, bmi;
	
	while(scanf("%d,%lf,%lf", &num, &wei, &hei)!=EOF){
		
		bmi=wei/hei/hei;
		
		if(bmi>=25.0)
		printf("%d\n", num);
	}
	return 0;
}