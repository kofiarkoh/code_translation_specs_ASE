#include <stdio.h>
#include <stdlib.h>
#include <math.h>


int main()
{
	int n;
	int bit[19];
	int *t, t_n;
	int x, tmp;
	int size;
	int flag;
	int i, j, k;

	scanf("%d", &n);

	scanf("%d", &t_n);

	t = (int *)malloc(sizeof(int) * t_n);
	for(i = 0; i < t_n; i++)
		scanf("%d", &t[i]);

	for(i = 0; i < pow(2, n); i++){
		x = i;
		j = k = 0;
		while( (tmp = x / 2) >= 1){
			if(x % 2)
				bit[k++] = j;
			x = tmp;
			j++;
		}
		if(x)
			bit[k] = j;

		size = k;

		if(i > 0){
			flag = 1;
			for(j = 0; j < t_n; j++){
				for(k = 0; k <= size; k++)
					if(t[j] == bit[k])
						break;
				if(k == size + 1){
					flag = 0;
					break;
				}
			}

			if(flag){
				printf("%d:", i);
				for(j = 0; j <= size; j++)
					printf(" %d", bit[j]);
				printf("\n");
			}
		}else if(i == 0 && t_n == 0)
			printf("0:\n");
	}

	return 0;
}

