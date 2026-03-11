#include <stdio.h>

int main(void)
{
	float m, hm, lm;
	int first = 0;
	while (scanf("%f", &m) != EOF){
		if (first == 0){
			first = 1;
			hm = lm = m;
		}
		if (hm < m) hm = m;
		else if (lm > m) lm = m;
	}
	
	printf("%.1f\n", hm - lm);
	return(0);
}