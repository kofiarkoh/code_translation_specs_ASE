#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
int a[5000];
int main(){
	int n;
	scanf("%d",&n);
	for(int i=1;i<sqrt(n);i++){
		for(int j=1;j<sqrt(n-i*i);j++){
			for(int k=1;k<sqrt(n-i*i-j*j);k++){
				a[i*i+j*j+k*k+i*j+j*k+k*i]++;
			}
		}
	}
	for(int i=1;i<=n;i++){
		printf("%d\n",a[i]);
	}
	return 0;
} 