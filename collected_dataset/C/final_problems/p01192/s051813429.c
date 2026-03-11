#include <stdio.h>

int ans_greedy(const int* c,int pos,int price) {
	int answer;
	if(price<=0)return 0;
	answer=price/c[pos];
	if(pos>0)answer+=ans_greedy(c,pos-1,price%c[pos]);
	return answer;
}

int main(void) {
	int n;
	int c[50];
	int i;
	int case_no=1;
	while(1) {
		scanf("%d",&n);
		if(n==0)break;
		for(i=0;i<n;i++)scanf("%d",&c[i]);
		if(c[0]!=1) {
			printf("Case #%d: Cannot pay some amount\n",case_no);
		} else {
			for(i=1;i<n;i++) {
				if(c[i]%c[i-1]!=0) {
					int bai=c[i]/c[i-1]+1;
					int greedy=ans_greedy(c,i,c[i-1]*bai);
					if(greedy>bai)break;
				}
			}
			printf("Case #%d: %s\n",case_no,
				i>=n?"OK":"Cannot use greedy algorithm");
		}
		case_no++;
	}
	return 0;
}