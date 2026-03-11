#include <stdio.h>

int main(void)
{
	int n, t, x = 0, i, a = 0, b = 0;
	
	int botan[200000];
	
	scanf("%d %d", &n, &t);
	scanf("%d", &a);
	for(i = 0; i < n - 1; i++){
		scanf("%d", &b);
		if(b - a <= t){
			x += b - a;
			
		}else{
			x += t;
			
		}
		a = b;
		
		
	}
	x += t;
	printf("%d\n", x);
	
	return 0;
}