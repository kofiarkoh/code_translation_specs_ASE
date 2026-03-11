#include <stdio.h>

int main(void){
	int b, count, sum;
	int i, j;

	while(1){
		scanf("%d", &b);
		if(b==0) break;

		count = sum = 0;
		i = j = 1;
		while(1){
			if(sum == b) break;
			else if(sum < b){
				sum += j;
				count++;
				j++;
			} else{
				sum -= i;
				count--;
				i++;
			}
		}
		printf("%d %d\n", i, count);
	}//endwhile

	return 0;
}
