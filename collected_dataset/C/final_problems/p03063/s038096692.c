#include <stdio.h>
#include <string.h>

//白黒にする
//.....####みたいにする
//.白. #黒#

int main(void)
{
	
	int n;
	int num[300000] = {0};
	int index = 0;
	int index_kuro;
	int index_siro;
	int count;
	int i, j;
	char str[300000];
	
	scanf("%d%s", &n, str);
	
	str[n] = 'a';
	
	count = 0;
	if(str[0] == '#') {
		num[0] = 0;
		index++;
	}
	
	for(i = 0; i <= n; i++) {
		if(str[i] == '.') {
			count = 1;
			for(j = i + 1; j <= n; j++) {
				if(str[j] != '.') {
					break;
				}
				count++;
			}
			i = j - 1;
			num[index] = count;
			index++;
		} else if(str[i] == '#') {
			count = 1;
			for(j = i + 1; j <= n; j++) {
				if(str[j] != '#') {
					break;
				}
				count++;
			}
			i = j - 1;
			num[index] = count;
			index++;
		}
	}
	
	count = 0;
	index_kuro = 1;
	if(index <= 2) {
		printf("0\n");
		return 0;
	}
	if(index % 2 == 0) {
		index_siro = index - 2;
	} else {
		index_siro = index - 1;
	}
	
	while(index_kuro < index_siro) {
		if(num[index_kuro] > num[index_siro]) {
			count += num[index_siro];
			index_siro -= 2;
		} else {
			count += num[index_kuro];
			index_kuro += 2;
		}
	}
	
	printf("%d\n", count);
	
	return 0;
}