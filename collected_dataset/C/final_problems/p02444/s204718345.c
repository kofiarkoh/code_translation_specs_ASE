#include<stdio.h>
#define N 1000
#define EMPTY 2100000000

int main(){
	int n,q,i;
	int a[N];
	int copy[N];
	int b,m,e;
	
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
		copy[i]=EMPTY;
	}
	
	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d %d %d",&b,&m,&e);
		
		for(int j=0;j<(e-b);j++){
			copy[b+((j+e-m)%(e-b))]=a[b+j];
		}
		
	}
	
	for(i=0;i<n-1;i++){
		if(copy[i]!=EMPTY)printf("%d ",copy[i]);
		else printf("%d ",a[i]);
	}
	if(copy[n-1]!=EMPTY)printf("%d\n",copy[n-1]);
	else printf("%d\n",a[n-1]);
	
	return 0;
	
}
