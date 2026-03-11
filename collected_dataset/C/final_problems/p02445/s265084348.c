#include<stdio.h>
#define N 1000

void swap(int a[],int x,int y){
	int temp=a[x];
	a[x]=a[y];
	a[y]=temp;
	return;
}

int main(){
	int n,q,i,b,e,t;
	int a[N];
	
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%d",&a[i]);
	
	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d %d %d",&b,&e,&t);
		for(int j=0;j<(e-b);j++){
			swap(a,b+j,t+j);
		}
	}
	
	for(i=0;i<n-1;i++){
		printf("%d ",a[i]);
	}
	printf("%d\n",a[n-1]);
	
	return 0;
	
}
