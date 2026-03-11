#include <stdio.h>

int main(void) {
	double x[4],y[4];
	int yes;
	double katamuki,seppen;
	while(1) {
		if(scanf("%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf",
			&x[0],&y[0],&x[1],&y[1],&x[2],&y[2],&x[3],&y[3])==-1)break;
		yes=1;
		if(x[0]==x[2]) {
			if((x[1]-x[0])*(x[3]-x[0])>0)yes=0;
		} else if(y[0]==y[2]) {
			if((y[1]-y[0])*(y[3]-y[0])>0)yes=0;
		} else {
			katamuki=(y[0]-y[2])/(x[0]-x[2]);
			seppen=y[0]-katamuki*x[0];
			if((y[1]-(katamuki*x[1]+seppen))*(y[3]-(katamuki*x[3]+seppen))>0)
				yes=0;
		}
		if(x[1]==x[3]) {
			if((x[0]-x[1])*(x[2]-x[1])>0)yes=0;
		} else if(y[1]==y[3]) {
			if((y[0]-y[1])*(y[2]-y[1])>0)yes=0;
		} else {
			katamuki=(y[1]-y[3])/(x[1]-x[3]);
			seppen=y[1]-katamuki*x[1];
			if((y[0]-(katamuki*x[0]+seppen))*(y[2]-(katamuki*x[2]+seppen))>0)
				yes=0;
		}
		printf("%s\n",yes?"YES":"NO");
	}
	return 0;
}