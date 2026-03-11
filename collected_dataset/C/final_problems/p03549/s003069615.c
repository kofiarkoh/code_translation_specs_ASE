#include <stdio.h>

int main(void){
	int n, m, t=0, i, d=1;

	scanf("%d %d", &n, &m);

	t = 1900 * m + 100 * (n - m);
	for(i = 1; i <= m; i++){
        d *= 2;
	}

    printf("%d\n", t * d); 
    
    return 0;
}

