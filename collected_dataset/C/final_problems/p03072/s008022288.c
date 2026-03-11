#include <stdio.h>
#define T 20

int main(void) {
	int N, i,max,count=1;
	scanf("%d",&N);
	int H[T];
	for (i = 1; i < N + 1; i++) scanf("%d", &H[i]);
    max=H[1];
	for (i = 2; i < N + 1; i++) {
        if(max<=H[i]) {
          count++;
          max=H[i];
			}
		}
	

	printf("%d\n", count);
	return 0;
}


