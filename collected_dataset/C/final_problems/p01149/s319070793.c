#include <stdio.h>
#include <ctype.h>

int main(void) {
	int case_num,now_case;
	char cards[10][2];
	int i;
	int score,aces;
	scanf("%d",&case_num);
	for(now_case=0;now_case<case_num;now_case++) {
		for(i=0;i<10;i++)scanf("%s",cards[i]);
		score=aces=0;
		for(i=0;i<2;i++) {
			if(isdigit(cards[i][0])) {
				score+=cards[i][0]-'0';
			} else if(cards[i][0]=='A') {
				score+=11;
				aces++;
			} else {
				score+=10;
			}
			for(;aces>0 && score>21;aces--)score-=10;
		}
		if(score==21){puts("blackjack");continue;}
		for(i=2;i<10;i++) {
			if(score>17 || (score==17 && aces!=1))break;
			if(isdigit(cards[i][0])) {
				score+=cards[i][0]-'0';
			} else if(cards[i][0]=='A') {
				score+=11;
				aces++;
			} else {
				score+=10;
			}
			for(;aces>0 && score>21;aces--)score-=10;
		}
		if(score>21)puts("bust"); else printf("%d\n",score);
	}
	return 0;
}