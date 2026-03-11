#include <stdio.h>
#include <stdlib.h>
#include <math.h>


#define SUB_SIZE 19
#define	SIZE	pow(2, SUB_SIZE)	// 2^19 = 524,288


int main()
{
	int n, c;
	char *one_count;
	int count, tmp;
	int bk;
	int i, j, k;

	scanf("%d %d", &n, &c);

	// 信号の配列を用意
	one_count = (char *)malloc(sizeof(char) * SIZE);

	// 初期化
	for(i = 0; i < SIZE; i++)
		one_count[i] = 0;

	// 信号二次元配列の作成
	for(i = 0; i < SUB_SIZE; i++){
		tmp = pow(2, i);
		for(j = tmp; j < SIZE; j += 2 * tmp){
			count = 0;
			k = j;
			while(count < tmp){
				one_count[k]++;
				count++;
				k++;
			}
		}
	}


	// 全組み合わせの表示
	tmp = pow(2, n);
	for(i = 0; i < tmp; i++){
		if(one_count[i] == c){
			printf("%d:", i);
			bk = i;
			for(j = 0; j < n; j++, bk >>= 1)
				if(bk & 0x1)
					printf(" %d", j);
			printf("\n");
		}
	}

	return 0;
}
