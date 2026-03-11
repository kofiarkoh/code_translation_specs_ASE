#include <stdio.h>
#include<math.h>

int main(void){
	int i,j,k,z;
	int n=0;
	int count = 0;

	char s[100];
	scanf("%s", &s);
	while (s[n]!='\0'){
		n++;
	}
	if (n % 2 == 0) {
		j = n / 2;
		z = j - 1;
	}
	else {
		j = (n / 2) + 1;
		z = j - 2;
	}
	for (i = 0; i <= z; i++) {
		if (s[i] != s[n-1-i]) {
			count++;
		}
	}
	//printf("n=%d\nj=%d\nz=%d\n", n, j, z);
	printf("%d\n", count);
	return 0;
}
