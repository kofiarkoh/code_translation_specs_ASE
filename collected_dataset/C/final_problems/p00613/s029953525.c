#include <stdio.h>

int main(void){
	while(1){
		int k, t, i, sum=0;
		
		scanf("%d", &k);
		if(k==0) break;
		
		for(i=0; i<k*(k-1)/2; i++){
			scanf(" %d", &t);
			sum+=t;
		}
		sum/=k-1;
	
		printf("%d\n", sum);
	}

	return 0;
}