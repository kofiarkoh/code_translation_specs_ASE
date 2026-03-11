#include <stdio.h>
int main(void)
{
	int n,a;
	int i,sum=0,check=0;
	long long int count=0;
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&a);
		sum+=a;
		if(check==1&&sum>=0){
			count+=(1+sum);
			sum=-1;
		}else if(check==-1&&sum<=0){
			count+=(1-sum);
			sum=1;
		}
		if(sum>0){
			check=1;
		}else{
			check=-1;
		}	
	}
	printf("%lld",count);
	return 0;
}
