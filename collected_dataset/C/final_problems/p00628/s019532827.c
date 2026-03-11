#include <stdio.h>
#include <string.h>

int main(void)
{
	char str[50000];
	int temp;
	int i, j;
	
	while (1){
		scanf("%[A-Za-z ]", str);
		getchar();
		
		if (strcmp(str, "END OF INPUT") == 0){
			break;
		}
		
		temp = i = 0;
		while (str[i] != ' ' && str[i] != '\0'){
			temp++;
			i++;
		}
		printf("%d", temp);
		
		if (str[i] != '\0'){
			if (str[i] == ' ' && str[i + 1] == ' '){
				printf("0");
				i++;
			}
			i++;
			
			while (str[i] != '\0'){
				temp = 0;
				while (str[i] != ' ' && str[i] != '\0'){
					temp++;
					i++;
				}
				printf("%d", temp);
				
				if (str[i] == '\0'){
					break;
				}
				
				if (str[i] == ' ' && str[i + 1] == ' '){
					printf("0");
					i++;
				}
				i++;
			}
		}
		printf("\n");
	}
	
	return (0);
}