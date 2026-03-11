#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define lli long long int
#define min(a,b) ((a)<(b)?(a):(b))
#define max(a,b) ((a)>(b)?(a):(b))
#define true 1
#define false 0



int main(void){
	int i;
	char s[16];
	char a[16] = "CODEFESTIVAL2016";
	int count = 0;
	scanf("%s",s); 
	for(i=0;i<16;i++){
		if(s[i]!=a[i]){
			count++;
		}
	}
	printf("%d\n", count);
	return 0;
}