#include<stdio.h>
int main(){
	int n,k,a=1;
	scanf("%d%d",&n,&k);
	for(int i=0;i<n-1;i++){
		if(a*2<a+k){
			a=a*2;
			}
		if(a*2>a+k){
			a=a+k;
			}	
		}
	printf("%d",a);
	return 0;
	}