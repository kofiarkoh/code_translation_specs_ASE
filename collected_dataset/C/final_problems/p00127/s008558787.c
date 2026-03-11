#include <stdio.h>
#include <string.h>

int main(void)
{
	char str[256] = {0};
	
	while (fgets(str, 255, stdin) != NULL){
		int i;
		
		if (strlen(str) % 2 == 0){
			puts("NA");
			continue;
		}
		
		for (i = 0; str[i] != 10; i += 2){
			int dec = (str[i]  - '0') * 10 + str[i + 1] - '0';
			
			switch (dec){
				case 62: putchar('.'); break;
				case 63: putchar('?'); break;
				case 64: putchar('!'); break;
				case 65: putchar(' '); break;
				default: putchar((dec / 10 - 1) * 5 + dec % 10 + 96); break;
			}
		}
		putchar(10);
	}
	
	return 0;
}