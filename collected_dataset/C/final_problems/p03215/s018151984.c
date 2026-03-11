#include <stdio.h>

int main(void)
{
	int N,K,i,j,sum,ans;
	unsigned int a[1000];
	int max[2]={0};
	scanf("%d",&N);
	scanf("%d",&K);
	for(i=0;i<N;i++){
		scanf("%d",&a[i]);
	}
	
	for(i=0;i<N;i++){
		sum=0;
		for(j=i;j<N;j++){
			sum+=a[j];
		}
		if(sum>max[0]){
			max[1]=max[0];
			max[0]=sum;
		}else if(sum>max[1]){
			max[1]=sum;
		}
	}
	ans=max[0] & max[1];
	printf("%d",ans);
	return 0;
}