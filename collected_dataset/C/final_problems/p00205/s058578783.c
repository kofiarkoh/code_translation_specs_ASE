#include <stdio.h>

int main(void)
{
	int janken[5];
	char hantei[5];
	int i;
	int g, c, p;
	
	
	while(1){
		
		
		g = c = p = 0;
		
		for(i = 0; i < 5; i++){
			scanf("%d", &janken[i]);
			if(janken[0] == 0){
				break;
			}
			if (janken[i] == 1){
				g = 1;
				hantei[i] = 'g';
			}
			else if(janken[i] == 2){
				c = 1;
				hantei[i] = 'c';
			}
			else{
				p = 1;
				hantei[i] = 'p';
			}
		}
		if(janken[0] == 0){
			break;
		}
		
		if ((g == 1 && c == 0 && p == 0)||(g == 0 && c == 1 && p == 0)||(g == 0 && c == 0 && p == 1)){
			for (i = 0; i < 5; i++){
				janken[i] = 3;
			}
		}
		
		else if(g == 1 && c == 1 && p == 1){
			for(i = 0; i < 5; i++){
				janken[i] = 3;
			}
		}
		
		else {
			if (g == 1 && c == 1 && p == 0){
				for (i = 0; i < 5; i++){
					if (hantei[i] == 'g'){
						janken[i] = 1;
					}
					else{
						janken[i] = 2;
					}
				}
			}
			else if (g == 0 && c == 1 && p == 1){
				for (i = 0; i < 5; i++){
					if (hantei[i] == 'c'){
						janken[i] = 1;
					}
					else{
						janken[i] = 2;
					}
				}
			}
			else if (g == 1 && c == 0 && p == 1){
				for (i = 0; i < 5; i++){
					if (hantei[i] == 'p'){
						janken[i] = 1;
					}
					else{
						janken[i] = 2;
					}
				}
			}
		}
		
		for(i = 0; i < 5; i++){
			printf("%d\n", janken[i]);
		}
	}
	
	return (0);
}