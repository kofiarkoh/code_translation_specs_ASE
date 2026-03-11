#include <stdio.h>
int main(){
	int ans=0,n,k,x,a,i;
	scanf("%d%d%d",&n,&k,&x);
	a=x/1000;
	if(n<=k){
		if(x%1000==0)--a;
		ans=(a+1)*n;
	}
	else if(n<=k*2){
		if(x%1000==0)ans=n*a;
		else if(x%1000>500)ans=n*(a+1);
		else ans=n*a+k;
	}
	else{
		a=x/500+1;
		if(x%500==0)--a;
		ans=a*k;
	}
	printf("%d\n",ans);
	return 0;
}
