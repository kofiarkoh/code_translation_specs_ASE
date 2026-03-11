#include<stdio.h>

int main(void) {
	int i, k ,l;
	int kei;
	int niku;
	int touri;
	int la;
	int hon;
	int ans = 0;
	int sozai[200][2] = {};
	scanf("%d",&hon);

	for(int i = 0; i < hon; i++) {
		scanf("%d",&sozai[i][0]);
		scanf("%d",&sozai[i][1]);
	}
	for(i = 0; i < hon; i++) {
		for(k = i+1; k < hon; k++) {
			touri = 1;
			kei = sozai[i][0] + sozai[i][1] + sozai[k][0] + sozai[k][1];
			niku = sozai[i][0] + sozai[k][0];
			for(l = kei; l > 1; l--) {
				if(l <= niku || l  <= kei - niku) {
					break;
				}
				touri *= l;
			}
			la = niku > (kei - niku) ? (kei - niku) : niku;
			for(l = la ; l > 1; l--) {
				touri /= l;
			}
			ans += touri;
			if(ans > (1000000000 + 7)) ans -= 1000000000 + 7;
		}
	}
	printf("%d\n",ans);
	return 0;
}
