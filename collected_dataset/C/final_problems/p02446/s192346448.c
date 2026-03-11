#include <stdio.h>

#define EMPTY	(1000000001)

int main(void)
{
	int n;
	long now;
	long prev = EMPTY;
	
	scanf("%d", &n);
	while (n--){
		scanf(" %lld", &now);
		
		if (prev != now){
			
			if (prev == EMPTY){
				printf("%lld", now);
			}
			else {
				printf(" %lld", now);
			}
			
			prev = now;
		}
	}
	
	puts("");
	
	return (0);
}
