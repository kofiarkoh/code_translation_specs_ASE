#include <stdio.h>

int main() {
	int i = 0, k = 0, n, car[100] = {0}, out[100] = {0};
	while(scanf("%d", &n)!= EOF) {
		if(n == 0) {
			i--;
			out[k] = car[i];
			k++;
		} else {
			car[i] = n;
			i++;
		}
	}
	for(i = 0; i < k; i++) printf("%d\n", out[i]);
	return 0;
}