#include<stdio.h>
int main(void){
	int n,i,j,cnt=0;
	scanf("%d",&n);
	for(i=0;i<=n/7;i++){
			for(j=0;j<=n/4;j++){if(n==7*i+4*j){cnt=1;}}
	}
	printf(cnt?"Yes":"No");
	return 0;
}