#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#define N 13

int main(){
    int n,m;
    scanf("%d",&n);
	scanf("%d",&m);
	int a[101][101];
	int b[101];
int c[101];
    for(int i=0;i<n;i++){
		for(int j=0;j<m;j++){
			scanf("%d",&a[i][j]);
		}
	}
	for(int i=0;i<m;i++){
		scanf("%d",&b[i]);
	}
	for(int i=0;i<n;i++){
		c[i]=0;
		for(int j=0;j<m;j++){
			c[i]+=a[i][j]*b[j];
		}
		printf("%d\n",c[i]);
	}
}
