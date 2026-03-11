#include <stdio.h>

#define WIDTH 10
#define HEIGHT 10

int main(void) {
	int case_num,case_count;
	if(scanf("%d",&case_num)!=1)return 1;
	for(case_count=0;case_count<case_num;case_count++) {
		int search;
		int i,j;
		int status[HEIGHT];
		int answer[HEIGHT];
		for(i=0;i<HEIGHT;i++) {
			status[i]=0;
			for(j=0;j<WIDTH;j++) {
				int cur;
				if(scanf("%d",&cur)!=1)return 1;
				if(cur!=0)status[i]|=(1<<j);
			}
		}
		for(search=0;search<(1<<WIDTH);search++) {
			int prev_prev_status=0;
			int prev_status=search;
			int cur_status=0;
			for(i=0;i<HEIGHT;i++) {
				answer[i]=prev_status;
				cur_status=status[i]^prev_prev_status;
				if(prev_status&1)cur_status^=3;
				for(j=1;j<WIDTH;j++) {
					if(prev_status&(1<<j))cur_status^=7<<(j-1);
				}
				cur_status&=(1<<WIDTH)-1;
				prev_prev_status=prev_status;
				prev_status=cur_status;
			}
			if(cur_status==0)break;
		}
		for(i=0;i<HEIGHT;i++) {
			for(j=0;j<WIDTH;j++) {
				printf("%d%c",(answer[i]&(1<<j))?1:0,j+1<WIDTH?' ':'\n');
			}
		}
	}
	return 0;
}