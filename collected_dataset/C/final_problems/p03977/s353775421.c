#include<stdio.h>
int main(void){
	int t,n,d,r,i;
	scanf("%d",&t);
	for(i=1;i<=t;i++){
		scanf("%d%d",&n,&d);
		if(n%2){
			printf("%d\n",127*(n-1)+d);
		}
		else{
			printf("%d\n",127*(n-1)+(d^127));
		}
	}
	return 0;
}