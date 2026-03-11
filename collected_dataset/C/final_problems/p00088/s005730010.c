// AOJ Volume 0 Problem 0088

#include <stdio.h>
#include <string.h>

#define MAX_STR (500)

typedef struct {
	char c;
	char *code;
} TABLE1;

TABLE1 table1[] = {
{' ', "101"},
{'\'', "000000"},
{',', "000011"},
{'-', "10010001"},
{'.', "010001"},
{'?', "000001"},
{'A', "100101"},
{'B', "10011010"},
{'C', "0101"},
{'D', "0001"},
{'E', "110"},
{'F', "01001"},
{'G', "10011011"},
{'H', "010000"},
{'I', "0111"},
{'J', "10011000"},
{'K', "0110"},
{'L', "00100"},
{'M', "10011001"},
{'N', "10011110"},
{'O', "00101"},
{'P', "111"},
{'Q', "10011111"},
{'R', "1000"},
{'S', "00110"},
{'T', "00111"},
{'U', "10011100"},
{'V', "10011101"},
{'W', "000010"},
{'X', "10010010"},
{'Y', "10010011"},
{'Z', "10010000"},
};

char *table2 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ .,-'?";

int main(void)
{
	char input[MAX_STR];
	char code[MAX_STR * 8];
	TABLE1 *p;
	int len;
	int i, j;
	int n;

	while (fgets(input, MAX_STR, stdin) != NULL){
		code[0] = '\0';
		for (i = 0; input[i] != '\n'; i++){
			p = table1;
			while (p->c != input[i]){
				p++;
			}
			strcat(code, p->code);
//			printf("<%s>", p->code);
		}
//		printf("(%s)", code);
		
		len = strlen(code);
		strcat(code, "0000");

		len = ((len + 4) / 5) * 5;

//		printf("[%d]", len);
		
		i = 0;
		while (i < len){
			n = 0;
			for (j = 0; j < 5; j++){
				n = n * 2 + code[i] - '0';
				i++;
			}
			printf("%c", table2[n]);
		}
		printf("\n");
	}
	
	return (0);
}