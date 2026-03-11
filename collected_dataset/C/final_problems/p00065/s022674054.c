#include <stdio.h>

typedef struct {
	int la;
	int th;
} NUM;

int main(void)
{
	int id, i;
	char str[16];
	NUM num[2000] = {{0, 0}};
	
	while (fgets(str, 16, stdin), str[0] != '\n'){
		i = 0;
		id = 0;
		while (str[i] != ','){
			id = id * 10 + str[i] - '0';
			i++;
		}
		
		num[id].la++;
	}
	
	while (fgets(str, 16, stdin) != NULL){
		i = 0;
		id = 0;
		while (str[i] != ','){
			id = id * 10 + str[i] - '0';
			i++;
		}
		
		num[id].th++;
	}
	
	for (i = 0; i < 2000; i++){
		if (num[i].la > 0 && num[i].th > 0){
			printf("%d %d\n", i, num[i].la + num[i].th);
		}
	}
	
	return (0);
}