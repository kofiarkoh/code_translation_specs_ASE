#include <stdio.h>
int main(void) {
	int n,m,k = 0;
	//int a[100010];
	//int b[100010];
	int a = 0;
	int b = 0;
	unsigned long aSum[200010];
	unsigned long bSum[200010];
	int ai,bi = 0;
	int count = 0;
	
	//For binary search
	int mid = 0;
	int left = 0;
	int right = 0;
	
	aSum[0] = 0;
	bSum[0] = 0;
	
	scanf("%d%d%d",&n,&m,&k);
	for(ai=1;ai<n+1;ai++){
		scanf("%d",&a);
		aSum[ai] = aSum[ai-1] + a;
		//printf("aSum = %d\n",aSum[ai]);
		}
	for(bi=1;bi<m+1;bi++){
		scanf("%d",&b);
		bSum[bi] = bSum[bi-1] + b;
	}
	aSum[ai+1] = aSum[ai];
	bSum[bi+1] = bSum[bi];
	ai = 0;
	bi = 0;
	
	for(ai=1;ai<n+1;ai++){
		left = 1;
		right = m;
		while(left <= right){
			mid = (left + right)/2;
			if(aSum[ai]+bSum[mid] <= k){
				left = mid + 1;
				if(count < ai + mid){
					count = ai + mid;
					//printf("count = %d\n",count);
					//printf("ai = %d\n",ai);
					//printf("mid = %d\n",mid);
					
				}
			}else{
				right = mid - 1;
			}
			//printf("asum = %d\n",aSum[ai]);
			////printf("bsum = %d\n",bSum[m]);
		}
	}
	
	printf("%d",count);
	return 0;
}
