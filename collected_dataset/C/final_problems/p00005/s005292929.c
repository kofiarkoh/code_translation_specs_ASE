#include<stdio.h>

int main(){
	int a, b;
	int n, m;
	int temp;
	int GCD, LCM;
	while(scanf("%d %d",&b,&a)!=EOF){
		if(b<a){
			temp = b;
			b = a;
			a = temp;
		}
		n = a;
		m = b;
		while(n){
			temp = m;
			m = n;
			n = temp % n;
		}
		printf("%d %d\n",m,a*(b/m));
	}
}