#include <stdio.h>

int main(void)
{
	int m, f, b, a, c;
	
	scanf("%d" "%d" "%d", &m, &f, &b);
	
	a = b - m;
	if(a > 0){
		c = f - a;
		if(c < 0){
			printf("NA\n");
		}
		else{
			printf("%d\n",a);
		}
	}
	else{
		printf("0\n");
	}
	
	return 0;
}

