#include <stdio.h>

int rn[4], ln[4];

int main(void)
{
	double right, left;
	double EPS;
	int i;
	
	EPS = 1e-8;
	
	while (scanf("%lf %lf", &right, &left) != EOF){
		
		if (right >= 1.1){
			rn[0]++;
		}
		else if (right >= 0.6){
			rn[1]++;
		}
		else if(right >= 0.2){
			rn[2]++;
		}
		else {
			rn[3]++;
		}
		if (left >= 1.1){
			ln[0]++;
		}
		else if (left >= 0.6){
			ln[1]++;
		}
		else if(left >= 0.2){
			ln[2]++;
		}
		else {
			ln[3]++;
		}
	}
	for (i = 0; i < 4; i++){
		printf("%d %d\n", rn[i], ln[i]);
	}
	return (0);
}
			