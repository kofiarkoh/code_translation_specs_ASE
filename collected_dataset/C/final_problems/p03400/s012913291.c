#include <stdio.h>

int main(void)
{
	int n,d,x,memo,a[101];
	
	int count=0;
	
	scanf("%d%d%d",&n,&d,&x);
	
	for(int i=0;i<n;i++){
		scanf("%d",&(a[i]));
	}
	
	for(int i=0;i<n;i++){
		memo=a[i];
		while(1+memo<=d){
			memo=memo+a[i];
			count++;
		}
	}
	
	printf("%d\n",count+x+n);
	
	return 0;
}