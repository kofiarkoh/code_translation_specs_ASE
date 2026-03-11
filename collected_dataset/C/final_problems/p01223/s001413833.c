#include <stdio.h>

int main(void) {
	int d_num,now_d;
	int n;
	int height[100];
	int i;
	int u_max,d_max;
	int now;
	scanf("%d",&d_num);
	for(now_d=0;now_d<d_num;now_d++) {
		scanf("%d",&n);
		for(i=0;i<n;i++)scanf("%d",&height[i]);
		u_max=d_max=0;
		for(i=1;i<n;i++) {
			if(height[i]>height[i-1]) {
				now=height[i]-height[i-1];
				if(now>u_max)u_max=now;
			} else {
				now=height[i-1]-height[i];
				if(now>d_max)d_max=now;
			}
		}
		printf("%d %d\n",u_max,d_max);
	}
	return 0;
}