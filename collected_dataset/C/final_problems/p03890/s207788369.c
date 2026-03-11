#include <stdio.h>
#include <math.h>

long long stone[262145];
long long f[262145];

int main(void){
	long long n, game;
	long long i, j, t;
	
	scanf("%lld", &n);
	
	game = (int)(pow(2.0, (double)n));
	
	for (i = 0; i < game; i++){
		scanf("%lld", &stone[i]);
	}
	
	for (t = 0; t < n; t++){
		j = 0;
		for (i = 0; i < game; i = j+1){
			if (f[i] == 0){
				for (j = i+1; j < game; j++){
					if (f[j] == 0){
						if (stone[i] > stone[j]){
							stone[i] -= stone[j];
							f[j] = 1;
						}
						else if (stone[i] == stone[j]){
							f[j] = 1;
						}
						else {
							stone[j] -= stone[i];
							f[i] = 1;
						}
						break;
					}
				}
			}
			else if (i > 0) j++;
		}
	}
	
	for (i = 0; i < game; i++){
		if (f[i] == 0) printf("%lld\n", stone[i]);
	}
	
	return 0;
}