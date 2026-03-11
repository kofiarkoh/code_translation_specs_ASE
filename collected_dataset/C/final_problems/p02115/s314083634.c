#include <stdio.h>
#include <stdlib.h>

int max(int a, int b){
	if(a > b){
		return a;
	}
	else{
		return b;
	}
}

int main(){
	int N, i, m = 0, d = 0, ans = 0;
	scanf("%d", &N);
	int *a = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%d", &a[i]);
	}
	for(i = 1; i < N; i++){
		if(m == 1){
			if(a[i] > a[i - 1]){
				d++;
			}
			else if(a[i] < a[i - 1]){
				ans = max(ans, d);
				m = -1;
				d = 1;
			}
		}
		else if(m == -1){
			if(a[i] < a[i - 1]){
				d++;
			}
			else if(a[i] > a[i - 1]){
				ans = max(ans, d);
				m = 1;
				d = 1;
			}
		}
		else{
			if(a[i] > a[i - 1]){
				m = 1;
				d = 1;
			}
			else if(a[i] < a[i - 1]){
				m = -1;
				d = 1;
			}
		}
	}
	ans = max(ans, d);
	printf("%d\n", ans + 1);
	return 0;
}