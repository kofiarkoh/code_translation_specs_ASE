#include <stdio.h>

int main(void)
{
	int n, len, i;
	char oct[1028];
	
	while (scanf("%d", &n), n){
		len = sprintf(oct, "%o", n);
		for (i = 0; i < len; i++){
			printf("%c", "01235789"[oct[i] - '0']);
		}
		puts("");
	}
	return (0);
}