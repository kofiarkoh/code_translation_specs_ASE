#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include <math.h>
#include <stdbool.h>


//using ll = long long;
int ASC( void *a,  void *b) {
	return strcmp(*(const char **)a, *(const char **)b);//降順
}
int GCD(int a, int b) {	//最大公約数
	if (b == 0) return a;
	else return GCD(b, a % b);
}
int min(double a, double b ) {
	if (a < b) {
		return a;
	}
	else
		return b;
}
int max(int a, int b) {
	if (a < b)
		return b;
	else
		return a;
}
int main(void)
{
	char n[200000];
	scanf("%s",n);
	long len = strlen(n);
	for (long i = 0; i < len; i++) {
		if (i == 0 || n[i] == '?') {
			if (n[i + 1] == 'D' || n[i + 1] == '?')
				n[i] = 'P';
			else n[i] = 'D';
		}
		else if (n[i] == '?') { 
			if (n[i - 1] == 'P'|| n[i-1] == 'D')
				n[i] = 'D';
			else if (n[i + 1] == 'D' || n[i + 1] == '?')
				n[i] = 'P';
		}
		printf("%c",n[i]);
	}
	return 0;
}//pd pdp