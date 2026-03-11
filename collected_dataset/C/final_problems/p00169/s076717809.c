#include <stdio.h>
#include <string.h>

int main(void)
{
	char str[100];
	int i, score[2], len;
	
	while (1){
		fgets(str, 100, stdin);
		
		if (str[0] == '0'){
			break;
		}
		
		score[0] = 0;
		score[1] = 0;
		i = 0;
		
		len = strlen(str);
		
		while (i < len){
			if (str[i] == '1' && (str[i + 1] >= '0' && str[i + 1] <= '3')){
				score[0] += 10;
				score[1] += 10;
				i += 3;
			}
			else if (str[i] == '1'){
				score[0] += 1;
				score[1] += 11;
				i += 2;
			}
			else if (str[i] >= '2' && str[i] <= '9'){
				score[0] += (str[i] - '0');
				score[1] += (str[i] - '0');
				i += 2;
			}
		}
		
		for (i = 0; i < 2; i++){
			if (score[i] > 21){
				score[i] = 0;
			}
		}
		
		if (score[0] < score[1]){
			printf("%d\n", score[1]);
		}
		else {
			printf("%d\n", score[0]);
		}
	}
	
	return (0);
}