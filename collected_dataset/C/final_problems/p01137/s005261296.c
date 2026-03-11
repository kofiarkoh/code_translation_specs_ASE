#include <stdio.h>

int main(void) {
	int e,m,y = 0,z = 0;
	while(scanf("%d",&e) , e != 0){
		m = 1000001;
		for(z = 0;z*z*z <= e;z++){
			for(y = 0;z*z*z+y*y <= e;y++){
				if(z + y + (e - (z*z*z + y*y)) < m){m = z + y + (e - (z*z*z + y*y));}
			}
		}
		printf("%d\n",m);
	}
	return 0;
}