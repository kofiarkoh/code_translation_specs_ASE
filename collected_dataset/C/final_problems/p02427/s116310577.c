#include <stdio.h>
#include <math.h>


int main()
{
	int n;
	int x, tmp;
	int i, j;

	scanf("%d", &n);

	for(i = 0; i < pow(2, n); i++){
		printf("%d:", i);

		x = i;
		j = 0;
		while( (tmp = x / 2) >= 1){
			if(x % 2)
				printf(" %d", j);
			x = tmp;
			j++;
		}
		if(x)
			printf(" %d", j);

		printf("\n");
	}

	return 0;
}
