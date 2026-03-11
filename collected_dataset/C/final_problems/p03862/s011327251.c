#include <stdio.h>

int main(void){
	int n, x, a[100000] = {0}, b[100000] = {0};
	int sum = 0;

	scanf("%d%d", &n, &x);
	for(int i=0; i < n; i++) {
		scanf("%d", &a[i]);
		b[i] = a[i];
	}

	for(int i=0; i < n-1; i++) {
		if(a[i]+a[i+1] <= x)
			continue;

		while(a[i]+a[i+1] > x) {
			if(a[i]>a[i+1]) {
				a[i]--;
			}else{
				a[i+1]--;
			}
		}
	}

	for(int i=0; i < n; i++) {
		sum += b[i] - a[i];
	}
	printf("%d", sum);

	return 0;
}
