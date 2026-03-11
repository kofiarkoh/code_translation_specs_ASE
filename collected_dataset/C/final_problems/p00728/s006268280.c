#include <stdio.h>

int main(void)
{
	int hen = 0;
	int max = 0,min = 9999;
	int a = 0,i = 0;
	int sum = 0;
	int b[23];
	int j = 0;
	while(1){
		scanf("%d",&hen);
		if(hen == 0)break;
		
		for(i = 0;hen > i;i++){
			scanf("%d",&a);
			sum += a;
			if(a < min)min = a;
			if(a > max)max = a;
				
		}
		sum = sum -(max + min);
		b[j] = sum / (hen - 2);
		j++;
		max = 0;min = 9999;sum = 0;
		
	}
	for(i = 0;i < j;i++){
		printf("%d\n",b[i]);
	}
	return 0;
}