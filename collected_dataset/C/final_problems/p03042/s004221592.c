#include <stdio.h>
#include <string.h>
int main(void){
	int s;
	int x,y;

	scanf("%d",&s);

	x = s % 100;
	y = s / 100;

	if(x == 0 |y == 0){
		printf("NA\n");
	}else if(x <= 12 && y <= 12){
		printf("AMBIGUOUS\n");
	}else if(x > 12 && y<= 12){
		printf("MMYY\n");
	}else if(x <= 12 && y > 12){
		printf("YYMM\n");
	}else {
		printf("NA\n");
	}
}