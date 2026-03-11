#include <stdio.h>

int main(void)
{
	int y, m, d, date;
	
	while (scanf("%d%d%d", &y, &m, &d) != EOF){
		date = y * 10000 + m * 100 + d;
		
		if (date >= 19890108){
			printf("heisei %d %d %d\n", y - 1988, m, d);
		}
		else if (date >= 19261225){
			printf("showa %d %d %d\n", y - 1925, m, d);
		}
		else if (date >= 19120730){
			printf("taisho %d %d %d\n", y - 1911, m, d);
		}
		else if ( date >= 18680908){
			printf("meiji %d %d %d\n", y - 1867, m, d);
		}
		else {
			printf("pre-meiji\n");
		}
	}
	
	return (0);
}