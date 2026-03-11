#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>


int main()
{
	int i;
	long long int N;
	int keta = 0;
	long long int tasikame = 0;
	long long int ketaketa = 1;
	int kotae = 0;

	scanf("%llu", &N);
	unsigned long long int m = N;
	while (m != 0) {
			m /= 10;
			keta++;
			tasikame += 9 * ketaketa;
			ketaketa *= 10;
	}
	ketaketa /= 10;

	while (tasikame > N) {
		tasikame -= 1 * ketaketa;
	}
	while (tasikame != 0) {
		kotae += tasikame % 10;
		tasikame /= 10;
	}

	printf("%u", kotae);
	return 0;
}
/*受け取るｎ１６けたもある時は、unsigned long long int で宣言して、%lluで受け取る。*/