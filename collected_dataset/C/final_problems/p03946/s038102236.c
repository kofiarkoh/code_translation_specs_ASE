#include <stdio.h>
#include<string.h>

int main(void) {
	int n,t;
	scanf("%d %d", &n, &t);
	
	
	int a[n + 5];
	int minValue,maxValue,minIndex,maxIndex;
	for(int i = 1; i < n + 1; i++){
		scanf("%d", &a[i]);
	}
	minValue = a[1];
	maxValue = a[1];
	minIndex = 1;
	maxIndex = 1;

	for(int i = 2; i < n + 1; i++){
		if(minValue > a[i]){
			minValue = a[i];
			minIndex = i;
		} else if(maxValue < a[i]){
			maxValue = a[i];
			maxIndex = i;
		}
	}

	int dif[n + 5][n + 5];
	int difMin = a[1] - a[2];
	int icount = 0;
	int jcount = 0;
	int changeCount = 0;
	
	for (int i = 1; i < n ; i++){
		for(int j = i + 1; j < n +1; j++){
			dif[i][j] = a[i] - a[j];
			if(difMin >= dif[i][j]){
				difMin =dif[i][j];
			}
		}
	}
	
	for (int i = 1; i < n ; i++){
		for(int j = i + 1; j < n + 1; j++){
			if(difMin == dif[i][j]){
				changeCount++;
			}
		}
	}
	
	printf("%d\n",changeCount);
	



	return 0;
}
