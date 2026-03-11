#include <stdio.h>

int main(void)
{
	int w, l, h;
	int n;
	int i;
	int x;
	
	while (1){
		scanf("%d %d %d", &w, &l, &h);
		if (w == 0 && l == 0 && h == 0){
			break;
		}
		
		if (w > h){
			w += h;
			h = w - h;
			w -= h;
		}
		if (l > h){
			l += h;
			h = h -l;
			l -= h;
		}
		if (w > l){
			w += l;
			l = w - l;
			w -= l;
		}
		scanf ("%d", &n);
		
		for (i = 0; i < n; i++){
			scanf("%d", &x);
			
			if (w * l <= x * x / 2 * 3 && w <= x && l <= x * 2){
				printf("OK\n");
			}else {
				printf("NA\n");
			}
		}
	}
	
	return (0);
}