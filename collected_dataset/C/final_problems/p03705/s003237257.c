#include<stdio.h>
#include<stdlib.h>

int main(){
	int n,a,b;
	int ans=1;
	int i;
	
	if(scanf("%d%d%d",&n,&a,&b)==-1){
		printf("error");
		exit(1);
	}

	if(((n==1)&&(a!=b))||(b-a)<0){
		ans=0;
	}else if((b-a)>0){
		ans=(n-2)*b-(n-2)*a+1;
	}
	printf("%d\n",ans);

	return 0;
}