#include <stdio.h>
#include <stdlib.h>

int qsort_comp(const void* x,const void* y) {
	int* a=(int*)x;
	int* b=(int*)y;
	if(*a<*b)return 1;
	if(*a>*b)return -1;
	return 0;
}

int main(void) {
	int n;
	int cards[52];
	int sum;
	int i,j;
	int answer[52];
	int answer_pos[52];
	while(1) {
		scanf("%d",&n);
		if(n==0)break;
		sum=0;
		for(i=0;i<n;i++) {
			scanf("%d",&cards[i]);
			sum+=cards[i];
			answer_pos[i]=-1;
		}
		qsort(cards,n,sizeof(int),qsort_comp);
		for(i=0;i<n;i++) {
			for(j=answer_pos[i]+1;j<n;j++) {
				if(cards[j]>0 && (sum-cards[j])%cards[j]==0) {
					sum-=cards[j];
					answer[i]=cards[j];
					answer_pos[i]=j;
					cards[j]=0;
					break;
				}
			}
			if(j>=n) {
				if(i>0) {
					answer_pos[i]=-1;
					cards[answer_pos[i-1]]=answer[i-1];
					sum+=answer[i-1];
					i-=2;
				} else break;
			}
		}
		if(i<n)puts("No");
		else {
			for(i=n-1;i>0;i--) {
				printf("%d ",answer[i]);
			}
			printf("%d\n",answer[0]);
		}
	}
	return 0;
}