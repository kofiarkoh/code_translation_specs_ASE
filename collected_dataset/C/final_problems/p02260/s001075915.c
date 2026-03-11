#include <stdio.h>
#define N 100

int main(void)
{
	int i,n,j,t,k,A[N];
	int count=0;
	scanf("%d",&n);
	for(i=0;i<n;i++)
		scanf("%d",&A[i]);
		
	for(i=0;i<n-1;i++){
		for(j=i+1;j<n;j++){
			if(A[i]>A[j]){
				t=A[j];
				A[j]=A[i];
				A[i]=t;
				count++;
			}
		}
		
	}
	for(k=0;k<n-1;k++)
		printf("%d ",A[k]);
		printf("%d\n",A[k]);
	printf("%d\n",count/3);
	
	return 0;
}