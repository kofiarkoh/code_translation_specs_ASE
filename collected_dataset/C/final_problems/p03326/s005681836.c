#include<stdio.h>

long long A[1010][8];
long long bufArray[1010];

long long Origin[1010][3];

void q_sort(long long numbers[], long long left, long long right)
{
	long long pivot;
	long long l_hold, r_hold;

	l_hold = left;
	r_hold = right;
	pivot = numbers[left];
	while (left < right)
	{
		while ((numbers[right] >= pivot) && (left < right))
			right--;
		if (left != right)
		{
			numbers[left] = numbers[right];
			left++;
		}
		while ((numbers[left] <= pivot) && (left < right))
			left++;
		if (left != right)
		{
			numbers[right] = numbers[left];
			right--;
		}
	}
	numbers[left] = pivot;
	pivot = left;
	left = l_hold;
	right = r_hold;
	if (left < pivot)
		q_sort(numbers, left, pivot - 1);
	if (right > pivot)
		q_sort(numbers, pivot + 1, right);
}

int main() {
	int N, M;
	long long sum[8] = { 0 };
	long long sumMax = 0;
	long long bufSum=0;
	scanf("%d %d", &N, &M);
	for (int i = 0; i < N; i++) {
		scanf("%lld %lld %lld", &Origin[i][0], &Origin[i][1], &Origin[i][2]);
		A[i][0] = Origin[i][0] + Origin[i][1] + Origin[i][2];
		A[i][1] = Origin[i][0] * (-1) + Origin[i][1] + Origin[i][2];
		A[i][2] = Origin[i][0] + Origin[i][1] * (-1) + Origin[i][2];
		A[i][3] = Origin[i][0] + Origin[i][1] + Origin[i][2] * (-1);
		A[i][4] = Origin[i][0] * (-1) + Origin[i][1] * (-1) + Origin[i][2];
		A[i][5] = Origin[i][0] + Origin[i][1] * (-1) + Origin[i][2] * (-1);
		A[i][6] = Origin[i][0] * (-1) + Origin[i][1] + Origin[i][2] * (-1);
		A[i][7] = Origin[i][0] * (-1) + Origin[i][1] * (-1) + Origin[i][2] * (-1);
	}
	for (int i = 0; i < 8; i++) {
		for (int j = 0; j < N; j++) {
			bufArray[j] = A[j][i];
		}
		q_sort(bufArray, 0, N - 1);
		bufSum = 0;
		for (int j = N - 1; j >= N - M; j--) {
			bufSum += bufArray[j];
		}
		if (sumMax < bufSum)sumMax = bufSum;
	}
	
	printf("%lld\n", sumMax);

	return 0;
}