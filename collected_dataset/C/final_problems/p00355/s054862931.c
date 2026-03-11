#include<stdio.h>
int main(void)
{
	int a,b,n,s,f,i,x,cnt;
	cnt=0;
	scanf("%d %d",&a,&b);
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d %d",&s,&f);
		if(a<f&&b>s){
			cnt++;
		}
	}
	if(cnt>0){
		printf("%d\n",1);
	}
	else{
		printf("%d\n",0);
	}
	return 0;
}

