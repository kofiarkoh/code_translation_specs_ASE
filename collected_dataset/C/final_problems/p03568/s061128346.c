#include <stdio.h>
#include <math.h>

int main(void){
	int N;
	scanf("%d", &N);
	int a[10];
	for(int i=0; i<N; i++){
		scanf("%d", a+i);
	}

	int num=0; // にている列の数
	int odd=0, even=0; // a内の偶数、奇数の数
	int extra=1; // 似ている列のうち積が奇数のもの

	num = (int) pow(3.0, N);

	for(int i=0; i<N; i++){
		if(a[i]%2==0){
			even++;
		}
		else{
			odd++;
		}
	}

	extra = (int) (pow(1.0, odd) * pow(2.0, even));

	/*for(int i=0; i<N; i++){
		printf("%d\n", a[i]);
	}
	printf("%d %d %d\n", num, odd, even);*/

	printf("%d\n", num-extra);
	return 0;
}
