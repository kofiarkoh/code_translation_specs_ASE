#include <stdio.h>
#include <math.h>

int main(void){
	int ans = 0;
	char price[5];
	int len;
	int num;
	int tmp;
	int flag[10] = {};
	int i, j, k;
	scanf("%s", price);
	scanf("%d", &num);
	//printf("%s\n", price);
	for(i = 0; i < num; i++){
		scanf("%d", &tmp);
		flag[tmp] = 1;
	}
	for(i = 0; price[i] != '\0'; i++);
	len = i;
	for(i = 0; i < len; i++){
		for(j = price[i] - '0'; j < 10; j++){
			if(flag[j] == 0){
				if(j == 10){
					if(i != 0){
						price[i - 1] += 1;
					} else {
						for(k = 1; flag[k] == 1; k++){
							ans = k * pow(10, 4);
						}
					}
				}
				break;
				
			}
		}
		price[i] = j + '0';
		
	}

	//ans = 0;
	for(i = 0; i < len; i++){
		ans += (int)(price[i] - '0') * pow(10, len - 1 - i);
	}
	printf("%d\n", ans);
}