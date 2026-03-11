#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#define SIZE 100000

void QuickSort(int left, int right, int array[]) {
	int left0 = left, right0 = right;
	int buff;
	int pivot;
	if (left >= right)return;
	pivot = (array[left] + array[right] + array[(right + left) / 2]) / 3;
	while (1) {
		while (array[left] < pivot)left++;
		while (array[right] > pivot)right--;
		if (left >= right)break;
		buff = array[left];
		array[left] = array[right];
		array[right] = buff;
		left++;
		right--;
	}
	QuickSort(left0, left - 1, array);
	QuickSort(right + 1, right0, array);
}

int main() {
	int flag;
	int n, k, P;
	int W[SIZE];	//size : n
	int T[SIZE];	//size : k
	scanf("%d %d", &n, &k);
	for (int i = 0; i < n; i++) {
		scanf("%d", &W[i]);
	}
	QuickSort(0, n - 1, W);
	P = W[n - 1];
	while (1) {
		for (int j = 0; j < k; j++) {
			T[j] = P;
		}
		flag = 0;
		for (int num = n-1; num >= 0; num--) {
			for (int j = 0; j < k; j++) {
				if (T[j] >= W[num]) {
					T[j] -= W[num];
					break;
				}
				else if (j == k - 1) {
					flag = 1;
					break;
				}
			}
			if (flag == 1)break;
		}
		if (flag == 1)P++;
		else {
			printf("%d\n", P);
			break;
		}
	}
	return 0;
}
