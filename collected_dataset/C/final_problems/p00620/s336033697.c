#include <stdio.h>
int main(void) {
	int num,sum,i,temp;
	while (1) {
		scanf("%d",&num);
		if (num==0) break;
		sum=0;
		for (i=0; i<num*num; i++) {
			scanf("%d",&temp);
			sum+=temp;
		}
		if (sum==0) {
			printf("YES\n");
		} else {
			printf("NO\n");
		}
	}
	return 0;
}