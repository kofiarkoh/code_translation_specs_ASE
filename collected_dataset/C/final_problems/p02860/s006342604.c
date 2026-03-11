#include <stdio.h>
#include <stdlib.h>

int main(void) {
	int n, i;
	char s[101]= {0};

	scanf("%d", &n);
	scanf("%s", s);

	if(n%2 != 0){
		printf("No");
		return EXIT_SUCCESS;
	}

	for(i=0; i < n/2;i++){
		if(s[i] != s[i+n/2]){
			printf("No");
			return EXIT_SUCCESS;
		}
	}

	printf("Yes");

	return EXIT_SUCCESS;
}
