#include <stdio.h>
#include <math.h>

int main(){
	
	int x, a, b;
	scanf( "%d %d %d", &x, &a, &b );
	
	if( fabs((double)(x-a)) < fabs((double)(x-b)) ){
		printf("A");
	}else{
		printf("B");
	}
	
	return 0;
	
}