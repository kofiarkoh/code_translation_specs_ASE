#include <stdio.h>

int main(void) {
	int x, y;
	int button = 0;
	
	scanf("%d %d", &x, &y);
	
	if (y == x*-1) button = 1;
	
	if (x < y){
		if (x >= 0 || (x < 0 && y < 0)) button = y - x;
		else if (x*-1 < y) button = y - x*-1 + 1;
		else button = x*-1 - y + 1;
	}
	else {
		if ((x > 0 && y > 0) || (x < 0 && y < 0)) button = x - y + 2;
		else if (x == 0) button = y*-1 + 1;
		else if (y == 0) button = x+1;
		else button = x + y + 1;
	}
	
	printf("%d\n", button);
	
	return 0;
}