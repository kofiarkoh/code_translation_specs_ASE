#include <stdio.h>

int main(void){
	double ave = 0.0;

	while(1){
		int n;
		scanf("%d", &n);
		if(n==0) break;

		static int a[10001];
		for(int i=0; i<n; i++){
			scanf("%d", &a[i]);
		}

		ave = 0.0;
		for(int i=0; i<n; i++){
			ave += a[i];
		}
		ave = ave/(double)n;

		int count = 0;
		for(int i=0; i<n; i++){
			if(a[i] <= ave) count++;
		}

		printf("%d\n", count);
	}

	return 0;
}
