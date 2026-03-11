#include <stdio.h>
#include <math.h>

int main(){
	float v,t,y;
	while(scanf("%f",&v) != EOF){
		t = v / 9.8;
		y = t * t * 4.9;
		printf("%d\n",(int)ceilf(y / 5.0) + 1);
	}
	return 0;
}