#include <stdio.h>

int a[25], n, min;

void solve(int s, int p, int g)
{
	int i;
	
	if (n == s){
		
		if (p < g){
			i = g - p;
		}
		else {
			i = p - g;
		}
		if (min > i){
			min = i;
		}
		
		return;
	}
	
	solve (s + 1, p + a[s], g);
	solve (s + 1, p, g + a[s]);
}
	

int main(void)
{
	int i;
	
	while(1){
		scanf ("%d", &n);
		if (n == 0){
			break;
		}
		min = 20000005;
		for (i = 0; i < n; i++){
			scanf ("%d", &a[i]);
		}
		
		solve (0, 0, 0);
		
		printf("%d\n", min);
	}
	
	return (0);
}