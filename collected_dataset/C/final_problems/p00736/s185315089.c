#include <stdio.h>
#include <ctype.h>
#include <string.h>
#define or(a,b) a > b ? a : b
#define and(a,b) a > b ? b : a
#define not(n) 1 - (n - 1)

int p[3];
char str[81];

int formula(int i){
	if(isupper(str[i]))
		return p[str[i] - 'P'];
	else if(isdigit(str[i]))
		return str[i] - '0';
	else if(str[i] == '-')
		return not(formula(i + 1));
	else if(str[i] == '('){
		int j = i;
		int cen = 0;
		while(1){
			if(str[++j] == '(')
				cen++;
			else if(str[j] == ')')
				cen--;
			if(cen == 0 && (str[j] == '*' || str[j] == '+')){
				if(str[j] == '*')
					return and(formula(i + 1),formula(j + 1));
				else
					return or(formula(i + 1),formula(j + 1));
			}
		}
	}
}

int main(void){
	int count;
	gets(str);
	while(str[0] != '.'){
		str[strlen(str)] = '\0';
		count = 0;
		for(p[0] = 0;p[0] <= 2;p[0]++){
			for(p[1] = 0;p[1] <= 2;p[1]++){
				for(p[2] = 0;p[2] <= 2;p[2]++){
					if(formula(0) == 2)
						count++;
				}
			}
		}
		printf("%d\n",count);
		gets(str);
	}
	return 0;
}