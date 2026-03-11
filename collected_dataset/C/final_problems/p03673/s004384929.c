#include<stdio.h>
#include<stdlib.h>
#define N 2e5+5
void reverse(int a[],int k){
	int temp;
	for (int i=0,j=k-1;i<j;i++,j--){
		temp = a[i];
		a[i]=a[j];
		a[j]=temp;
	}
}
int main(){
	int n ;
	int a[(int)N];
	int b[(int)N];
	while(scanf("%d\n",&n)==1){
	for( int i =0;i<n;i++){
		scanf("%d",&a[i]);
	}
	for (int j=0;j<n;j++){
		b[j]=a[j];
		reverse(b,j+1);
	}
	for ( int i =0;i<n;i++){
		printf("%d ",b[i]);
	}
	printf("\n");
}
}