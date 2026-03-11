#include <stdio.h>
#include <string.h>

int main(void)
{
	char str[2000], cpy[2000];
	int len, i;
	
	fgets(str, 2000, stdin);
	
	len = strlen(str);
	
	for (i = 0; i < len; i++){
		if (strncmp(&str[i], "apple", 5) == 0){
			strncpy(&cpy[i], "peach", 5);
			i += 4;
		}
		else if (strncmp(&str[i], "peach", 5) == 0){
			strncpy(&cpy[i], "apple", 5);
			i += 4;
		}
		else cpy[i] = str[i];
	}
	
	printf("%s", cpy);
	
	return (0);
}