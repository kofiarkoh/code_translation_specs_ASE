#include <stdio.h>

int main(void)
{
	int r;
	int c;
	
	scanf("%d%d", &r, &c);
	
	while(r != 0 && c != 0) {
		if(r * c % 2 == 0) {
			printf("yes\n");
		}
		else {
			printf("no\n");
		}
		
		scanf("%d%d", &r, &c);
	}
	
	return (0);
}