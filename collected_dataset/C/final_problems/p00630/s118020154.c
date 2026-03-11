#include <stdio.h>
#include <ctype.h>

int main(void)
{
	char name[200];
	char rule[10];
	char word[100][100];
	int pos, n;
	int i, j;
	
	while (1){
		scanf("%s", name);
		scanf("%s", rule);
		if (rule[0] == 'X'){
			break;
		}
		
		pos = 0;
		n = 0;
		while (name[pos] != '\0'){
			i = 0;
			do {
				word[n][i++] = name[pos++];
			} while (name[pos] != '_' && ('a' <= name[pos] && name[pos] <= 'z'));
			
			word[n][i] = '\0';
			
			if (name[pos] == '_'){
				pos++;
			}
			n++;
		}
		
		switch (rule[0]){
		  case 'L':
			printf("%c", tolower(word[0][0]));
			printf("%s", &word[0][1]);
		  	for (i = 1; i < n; i++){
				printf("%c", toupper(word[i][0]));
				printf("%s", &word[i][1]);
		  	}
			printf("\n");
			break;
			
		  case 'U':
		  	for (i = 0; i < n; i++){
				printf("%c", toupper(word[i][0]));
				printf("%s", &word[i][1]);
		  	}
			printf("\n");
			break;

		  case 'D':
			printf("%c", tolower(word[0][0]));
			printf("%s", &word[0][1]);
		  	for (i = 1; i < n; i++){
				printf("_%c", tolower(word[i][0]));
				printf("%s", &word[i][1]);
		  	}
			printf("\n");
			break;
		}
	}
	
	return (0);
}