#include<stdio.h>
#include<math.h>
#include<string.h>
int main() {
	int n,a,b;
	int x[10000];
	scanf("%d %d %d",&n,&a,&b);
	for(int i=0; i<n; i++) {
		scanf("%d",&x[i]);
	}
	int sum=0;
	for(int i=1; i<n; i++) {
		if(a*(x[i]-x[i-1])<b) {
			sum+=a*(x[i]-x[i-1]);
		} else {
			sum+=b;
		}
	}
	printf("%d",sum);
	return 0;
}