#include<stdio.h>
int main(void)
{
	int n, check, i, a, b;
	scanf("%d",&n);
	int count[n];
	for(i=0;i<=n-1;i++){
		count[i] = 0;
	}
	for(i=0;i<=n-2;i++){
		scanf("%d %d",&a,&b);
		count[a]++;
		count[b]++;
	}
	check=0;
	for(i=0;i<=n-1;i++){
		if(count[i]==1){
			check++;
		}
	}
	if(((n-check-1)%3)==2){
		printf("Second");
	}else{
		printf("First");
	}
	return 0;
}