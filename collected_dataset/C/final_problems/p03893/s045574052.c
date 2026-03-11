#include<stdio.h>
#include<stdlib.h>
#include<stdlib.h>
int g(int n){
	if(n==0){
		return 2;
	}
	else{
		return g(n-1)*2+2; 
	}
}
int main(){
	int n;
	scanf("%d",&n);
	printf("%d",g(n));
	return 0;
}