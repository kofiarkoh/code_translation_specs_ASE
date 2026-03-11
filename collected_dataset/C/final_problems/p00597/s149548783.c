#include <stdio.h>

int main(void) {
	int sum;
	int query;
	int i;
	while(scanf("%d",&query)==1) {
		sum=0;
		for(i=1;i<=query;i++) {
			sum+=1+((query-i+1>i?i:query-i+1)-1)*2;
		}
		printf("%d\n",sum);
	}
	return 0;
}