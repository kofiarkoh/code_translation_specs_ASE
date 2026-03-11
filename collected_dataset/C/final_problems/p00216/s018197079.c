#include<stdio.h>

int main(void){
	int w, money;
	while(1) {
		scanf("%d",&w);
		if(w == -1) {
			break;
		} else if(w <= 10) {
			money = 1150;
		} else if(w <= 20) {
			money = 1150 + (w - 10) * 125;
		} else if(w <= 30) {
			money = 1150 + 10 * 125 + (w - 20) * 140;
		} else if(w <= 40) {
			money = 1150 + 10 * 125 + 10 * 140 + (w - 30) * 160;
		}
		printf("%d\n",4280 - money);
	}
	return 0;
}