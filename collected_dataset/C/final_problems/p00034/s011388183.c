#include <stdio.h>

int main(void){
	int l[10];
	int interval[11];
	int v1, v2;
	int x;
	double y;
	int i;

	while(1){
		if(scanf("%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d", &l[0], &l[1], &l[2], &l[3], &l[4], &l[5], &l[6], &l[7], &l[8], &l[9], &v1, &v2) == EOF) break;
		x = 0;
		interval[0] = 0;
		for(i=0; i<10; i++){
			interval[i+1] = interval[i] + l[i];	
			x += l[i];
		}
		y = (1.0 * v1 * x) / (v1 + v2);
		for(i=0; i<10; i++){
			if(interval[i] < y && y <= interval[i+1]){
				printf("%d\n", i+1);
			}
			if(y == interval[0]) puts("1");
		}
		
	}
	return 0;
}