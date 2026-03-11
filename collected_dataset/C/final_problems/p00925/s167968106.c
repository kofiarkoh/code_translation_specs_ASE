#include <stdio.h>

int eval1(char *exp)  // multiplication-first rule
{
	int result;
	int factor;
	int p;
	
	p = 0;
	
	result = 0;
	factor = exp[p++] - '0';
	
	while (exp[p] != '\0'){
		if (exp[p] == '+'){
			p++;
			result += factor;
			factor = exp[p++] - '0';
		}
		else if (exp[p] == '*'){
			p++;
			factor *= exp[p++] - '0';
		}
	}
	result += factor;
	
	return (result);
}

int eval2(char *exp)  //  left-to-right rule
{
	int result;
	int p;
	
	p = 0;
	
	result = exp[p++] - '0';

	while (exp[p] != '\0'){
		if (exp[p] == '+'){
			p++;
			result += exp[p++] - '0';
		}
		else if (exp[p] == '*'){
			p++;
			result *= exp[p++] - '0';
		}
	}

	return (result);
}

int main(void)
{
	char exp[20];
	int res1, res2;
	int bob;
	
	scanf("%s", exp);
	scanf("%d", &bob);
	
	res1 = eval1(exp);
	res2 = eval2(exp);

// printf("<%d %d>\n", res1, res2);
	
	
	if (res1 == bob && res2 == bob){
		printf("U\n");
	}
	else if (res1 == bob && res2 != bob){
		printf("M\n");
	}
	else if (res1 != bob && res2 == bob){
		printf("L\n");
	}
	else {
		printf("I\n");
	}
	
	return (0);
}