#include <stdio.h>
int main(void) {
	int n,i;
	while(scanf("%d",&n)!=EOF){
		int p[100];
		for(i=0;i<n;i++){
			scanf("%d",&p[i]);
		}
		int j,tmp;
		for(j=n-1;j>0;j--){
			for(i=0;i<j;i++){
				if(p[i]>p[i+1]){
					tmp=p[i];
					p[i]=p[i+1];
					p[i+1]=tmp;
				}
			}
		}
		int q[100];
		q[0]=p[0];
		for(i=1;i<n;i++){
			q[i]=q[i-1]+p[i];
		}
		int sum=0;
		for(i=0;i<n;i++){
			sum=sum+q[i];
		}
		printf("%d\n",sum);
	}
	return 0;
}

