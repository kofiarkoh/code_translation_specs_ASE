#include <stdio.h>

int main()
{
	int i, j, x;
	char str[100];

	while(gets(str)){
		int num[100] = {'\0'};

		i = 0;
		j = 0;

		while(str[i] != '\0'){
			switch(str[i]){
				case 'I':
					x = 1;
					break;
				case 'V':
					x = 5;
					break;
				case 'X':
					x = 10;
					break;
				case 'L':
					x = 50;
					break;
				case 'C':
					x = 100;
					break;
				case 'D':
					x = 500;
					break;
				case 'M':
					x = 1000;
					break;
				default:
					return 0;
			}

			if(i == 0) num[j] = x;
			else if(str[i] == str[i-1]) num[j] += x;
			else {j++; num[j] = x;}

			i++;
		}

		i = 0;
		x = 0;
		while(num[i] != '\0'){
			if(num[i] < num[i+1]) x -= num[i];
			else x += num[i];

			i++;
		}

		printf("%d\n", x);
	}

	return 0;
}