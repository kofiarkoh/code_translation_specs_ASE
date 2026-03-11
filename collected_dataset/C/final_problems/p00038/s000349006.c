#include <stdio.h>

int isstraight(int[5]);

int main(void) {
	int cards[5];
	int i,j,temp;
	int yaku;
	char yakuname[7][11]={"null","one pair","two pair","three card",
						"straight","full house","four card"};
	while(1) {
		if(scanf("%d,%d,%d,%d,%d",
			&cards[0],&cards[1],&cards[2],&cards[3],&cards[4])==-1)break;
		for(i=4;i>0;i--) {
			for(j=0;j<i;j++) {
				if(cards[j]>cards[j+1]) {
					temp=cards[j];
					cards[j]=cards[j+1];
					cards[j+1]=temp;
				}
			}
		}
		yaku=0;
		if(cards[1]==cards[2] && cards[2]==cards[3]) {
			if(cards[0]==cards[1] || cards[3]==cards[4])yaku=6;
			else yaku=3;
		} else if(cards[0]==cards[1] && cards[1]==cards[2]) {
			if(cards[3]==cards[4])yaku=5; else yaku=3;
		} else if(cards[2]==cards[3] && cards[3]==cards[4]) {
			if(cards[0]==cards[1])yaku=5; else yaku=3;
		} else if(isstraight(cards))yaku=4;
		else {
			for(i=0;i<4;i++) {
				if(cards[i]==cards[i+1])yaku++;
			}
		}
		printf("%s\n",yakuname[yaku]);
	}
	return 0;
}

int isstraight(int cards[5]) {
	int i;
	for(i=0;i<4;i++) {
		if(cards[i]+1!=cards[i+1])break;
	}
	if(i<4) {
		if(cards[0]==1 && cards[1]==10 && cards[2]==11 &&
			cards[3]==12 && cards[4]==13)return 1; else return 0;
	} else return 1;
}