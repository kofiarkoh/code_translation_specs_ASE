#include<stdio.h>
int main(void)
{
	int N, k; 

	//printf("Input N:");
	scanf("%d", &N); 

	int a[3*(10^5)];

	for (k = 0; k < 3 * N; k++)
	{
		//printf("input a[%d]:", k+1);
		scanf("%d", &a[k]);
	}

	//バブルソート降順
	int i, j, temp;

		for (i = 0; i<3*N; i++){
			for (j = 3*N - 1; j>i; j--){
				if (a[j] > a[j - 1]){
					temp = a[j];
					a[j] = a[j - 1];
					a[j - 1] = temp;
				}
			}
		}
	//ソート終了　

		long long sum = 0;
		int p;

		for (p = 0; p <=N-1; p+=1)
		{
			//printf("%d ", a[2 * (p + 1) - 1]);
			sum = sum+a[2*(p+1)-1];
		}
		printf("%lld",sum);

	return 0;
}