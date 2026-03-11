#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int main() {
	char S[100100];
	
	scanf("%s", S);
	int i, j;
	for(i = 0; i < strlen(S); i++) {
		if (S[i] == 'd') {
			if(S[++i] != 'r') {
				printf("NO");
				return 0;
			}
			if(S[++i] != 'e') {
				printf("NO");
				return 0;
			}
			if(S[++i] != 'a') {
				printf("NO");
				return 0;
			}
			if(S[++i] != 'm') {
				printf("NO");
				return 0;
			}
			if(S[i+1] == 'e') {
				if(S[i+2] == 'r') {
					if(S[i+3] != 'a') {
						i += 2;
					}
				}
			}
		} else if (S[i] == 'e') {
			if(S[++i] != 'r') {
				printf("NO");
				return 0;
			}
			if(S[++i] != 'a') {
				printf("NO");
				return 0;
			}
			if(S[++i] != 's') {
				printf("NO");
				return 0;
			}
			if(S[++i] != 'e') {
				printf("NO");
				return 0;
			}
			if(S[i + 1] == 'r')
					i++;
		}
	}
	printf("YES");
	
	return 0;
}