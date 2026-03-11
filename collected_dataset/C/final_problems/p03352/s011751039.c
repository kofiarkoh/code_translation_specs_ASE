#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>
#include <limits.h>
#include <math.h>

int note[1001];
int x;

int main() {
	scanf("%d", &x);

	// printf("hey\n");
	for(int i = 1; i <= 40; i++) {
		for(int j = 2; j <= 10; j++) {
			int temp = (int)((double)pow(i, j) + 0.1);
			if(temp > 1000) {
				continue;
			}
			note[temp] = 1;
		}
	}

	int last = 1;
	for(int i = 0; i <= x; i++) {
		if(note[i] == 1) {
			last = i;
		}
	}

	printf("%d\n", last);
	return 0;
}
