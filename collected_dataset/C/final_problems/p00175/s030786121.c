#include <stdio.h>

void strrev(char *in, char *out);

int main(void)
{
	int i;
	int a;
	char in[100];
	char out[100];
	
	scanf("%d", &a);
	
	while (a != -1){
		
		i = 0;
		if(a == 0){
			printf("0\n");
		}
		
		else{
			while (a != 0){
				in[i] = a % 4 + '0';
				a /= 4;
				i++;
			}
			
			in[i] = '\0';
			strrev(in, out);
			
			printf("%s\n", out);
		}
		
		scanf("%d", &a);
	}
	return (0);
	
}

void strrev(char *in, char *out)
{
	int a;
	int i;
	
	a = 0;
	while (in[a] != '\0'){
		a++;
	}
	out[a] = '\0';
	
	i = 0;
	while (in[i] != '\0'){
		out[i] = in[a - 1];
		i++;
		a--;
	}
}