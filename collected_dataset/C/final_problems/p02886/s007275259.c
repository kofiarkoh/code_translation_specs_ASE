#include<stdio.h>
#include<stdlib.h>
int main(){
	int n,sum=0;
	scanf("%d",&n);
	int * arr = (int*)malloc(sizeof(int)*51);
	for(int i=0;i<n;i++){
		scanf("%d",&arr[i]);
	}
	for(int i=0;i<n-1;i++){
		for(int j=i+1;j<n;j++){
			sum += arr[i]*arr[j];
		}
	}
	printf("%d", sum);
}