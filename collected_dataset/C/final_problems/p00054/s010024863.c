#include <stdio.h>

int main() {
	int a,b,n,r;
	int cnt, i;
	while( scanf("%d %d %d", &a, &b, &n)!=EOF ){
		r = 0;
		for(i=0; i<=n; i++) {
			if (i>0) r += a/b;
			a = ( a - (a/b)*b )*10;
		}
		printf("%d\n", r);
	}
	return 0;
}