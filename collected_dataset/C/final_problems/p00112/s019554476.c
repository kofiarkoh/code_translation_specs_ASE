#include <stdio.h>
#include <stdlib.h>

int main(void){
	int n,n1,sum,i,temp,time;int *a;
	while(1){
	scanf("%d",&n);n1=n;
	if(n == 0)
		break;
	a=(int *)calloc(n+1,sizeof(int));sum=0;
	scanf("%d",&a[1]);n1--;
	while(n1){
		scanf("%d",&time);
		if(time == 0)
			break;
		i=1;
		while( time < a[i])
			i++;
		while(i != n+1){
			temp=time;
			time=a[i];
			a[i]=temp;
			i++;
		}n1--;
	}
	for(i=2;i<=n;i++){
		sum+=a[i]*(i-1);
	}
	printf("%d\n",sum);
	}
	return 0;
}