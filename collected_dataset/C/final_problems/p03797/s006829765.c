#include <stdio.h>

int main(void)
{
	int n,m,ans;
	scanf("%d%d",&n,&m);
	if(n>=m/2){
		ans=m/2;
	}
	else{
		ans=n;
		m=m-2*n;
		ans=ans+m/4;
	}
	
	printf("%d",ans);
		
		
	
	return 0;
}