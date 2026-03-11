#include<stdio.h>
int main(){
	int n;
	while(scanf("%d",&n)!=EOF){
		n=n%39;
		if(n==0)n=39;
		printf("3C%02d\n",n);
	}
	return 0;
}