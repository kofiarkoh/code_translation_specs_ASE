#include <stdio.h>
int main(void){
	char s[1000];
	int i = 0;
	scanf("%s",s);
	while (s[i] != '\0'){
		switch (s[i]){
		case '2':
			printf("k");
			break;
		case '3':
			printf("s");
			break;
		case '4':
			printf("t");
			break;
		case '5':
			printf("n");
			break;
		case '6':
			printf("h");
			break;
		case '7':
			printf("m");
			break;
		case '8':
			printf("y");
			break;
		case '9':
			printf("r");
			break;
		case '0':
			if (s[i + 1] != 'U') printf("w");
			break;
		}
		switch(s[i + 1]){
		case 'L':
			printf("i");
			break;
		case 'U':
			if (s[i] != '0') printf("u");
			break;
		case 'R':
			printf("e");
			break;
		case 'D':
			printf("o");
			break;
		default:
			printf("a");
			break;
		}
		if (s[i] == '0' && s[i + 1] == 'U') printf("nn");
		i+=2;
	}
	printf("\n");
	return 0;
}