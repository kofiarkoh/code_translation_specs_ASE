#include <stdio.h>
#include <string.h>

int main(void)
{
	char in[1001];
	int freq[26];
	int p;
	int s;
	int i;
	int base;
	
	scanf("%s", &in);
	
	if (strlen(in) < 8){
		printf("%s\n", in);
		return (0);
	}
	
	memset(freq, 0, sizeof(freq));
	base = 0;
	for (p = 0; in[p] != '\0'; p++){
		freq[in[p] - 'A']++;
		if (freq['A' - 'A'] == 2 &&
			freq['I' - 'A'] == 1 &&
			freq['D' - 'A'] == 1 &&
			freq['U' - 'A'] == 1 &&
			freq['N' - 'A'] == 2 &&
			freq['Y' - 'A'] == 1){

			
			printf("AIZUNYAN");
			
			memset(freq, 0, sizeof(freq));
			base = p;
//printf("[base=%d]", base);
		}
		else {
//printf("*");
			if (p - 8 >= base){
				freq[in[p - 7] - 'A']--;
				printf("%c", in[p - 7]);
//				printf("<%d-%d=%d %c>", p, base, p - 8, in[p - 8]);
			}
		}
	}
	s = 0;
	for (i = 0; i < 26; i++){
//printf("[%c %d]", i + 'A', freq[i]);
		s += freq[i];
	}
//printf("<%d>", s);
	printf("%s\n", &in[strlen(in) - s]);
	
	return (0);
}