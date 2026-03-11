#include <stdio.h>

/* {1,2,3} -> ... -> {3,2,1} */
int next_permutation(int arr[],int n) {
	int target;
	int i;
	int temp;
	for (i = n - 2; i >= 0; i--) {
		if (arr[i] < arr[i + 1]) break;
	}
	if (i < 0) return 0;
	target = i;

	for (i = 0; target + i + 1 < n - i - 1; i++) {
		temp = arr[target + 1 + i];
		arr[target + 1 + i] = arr[n - 1 - i];
		arr[n - i - 1] = temp;
	}

	for (i = target + 1; i < n; i++) {
		if (arr[i] > arr[target]) break;
	}
	temp = arr[i];
	arr[i] = arr[target];
	arr[target] = temp;
	return 1;
}

int main(void) {
	int HorieYui[9];
	int MorikuboSyotaro[9] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
	int i;
	int answer = 0;
	for (i = 0; i < 9; i++) {
		if (scanf("%d" ,&HorieYui[i]) != 1) return 1;
	}
	do {
		int ok = 1;
		for (i = 0; i < 9; i++) {
			if (HorieYui[i] != -1 && HorieYui[i] != MorikuboSyotaro[i]) {
				ok = 0;
				break;
			}
		}
		if (ok) {
			if (MorikuboSyotaro[0] +
			(MorikuboSyotaro[1] * 10 + MorikuboSyotaro[2]) +
			(MorikuboSyotaro[3] * 100 + MorikuboSyotaro[4] * 10 + MorikuboSyotaro[5]) ==
			(MorikuboSyotaro[6] * 100 + MorikuboSyotaro[7] * 10 + MorikuboSyotaro[8])) {
				answer++;
			}
		}
	} while (next_permutation(MorikuboSyotaro, 9));
	printf("%d\n", answer);
	return 0;
}