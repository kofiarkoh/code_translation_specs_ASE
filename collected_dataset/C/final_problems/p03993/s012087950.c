#include <stdio.h>

int main(){
	int N = 100000;
	scanf("%d\n", &N);
	int usagi[100000];

	for (size_t i = 0; i < N; i++) {
		scanf("%d", &usagi[i]);
	}
	int count = 0;
	for (size_t i = 0; i < N; i++) {
		for (size_t j = i+1; j < N; j++) {
			if(usagi[i] == j+1 && usagi[j] == i+1){
				count++;
				break;
			}
			if (usagi[i] == i)
				break;
			if(count >= N/2)
				break;
		}
	}
	printf("%d\n", count);
	return 0;
}
