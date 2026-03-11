#include <stdio.h>

int main() {
	int f, i, num, r[10], rs, v1, v2;
	double x;
	while(1) {
		f = 0;
		for( i=0; i<10; i++) if (scanf("%d,", &r[i])==EOF) {
				f = 1;
				break;
			}
		if (f==1) break;
		scanf("%d,%d", &v1, &v2);
		
		rs = 0;
		for( i=0; i<10; i++) rs+=r[i];
		
		x = (double)v1 / (double)(v1+v2) * (double)rs;
		
		rs = 0;
		num = 0;
		for( i=0; i<10; i++) {
			rs += r[i];
			if ( x<=(double)rs ) {
				num = i+1;
				break;
			}
		}
		printf("%d\n", num);
	}
	return 0;
}