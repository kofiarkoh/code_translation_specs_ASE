#include <stdio.h>
#include <limits.h>

int main(void){
	int x,y,z,x3,z3;
	int sum,max;

	while(scanf("%d",&z) && z){
		max = INT_MIN;
		z3 = z*z*z;

		for(x=1;(x3=x*x*x)<=z3;x++){
			for(y=1;(sum=x3+y*y*y)<=z3;y++){
				if(max < sum) max = sum;
			}
		}

		printf("%d\n",z3-max);
	}

	return 0;
}