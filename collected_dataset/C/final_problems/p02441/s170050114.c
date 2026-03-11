#include<stdio.h>
int main(void){
	int n,i,j,q,b,e,k,count=0;
	int a[1000];
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
	}
	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d %d %d",&b,&e,&k);
		for(j=b;j<e;j++){
			if(a[j]==k){
				count++;
			}
		}
		printf("%d\n",count);
		count=0;
	}
	return 0;
}
