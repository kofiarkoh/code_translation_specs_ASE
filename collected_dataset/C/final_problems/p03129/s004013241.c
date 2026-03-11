#include<stdio.h>
int main(void){
	int n,k;
	scanf("%d %d",&n,&k);
	if(n%2==0){
		if(k<=n/2){
			puts("YES");
		}else{
			puts("NO");
		}
	}else{
		if(k<=n/2 +1){
			puts("YES");
		}else{
			puts("NO");
		}
	}
	return 0;
}