#define _CRT_SECURE_NO_WARNINGS
//#define _USE_MATH_DEFINES
#include<stdio.h>
//#include<stdlib.h>
//#include<math.h>
//#include<string.h>
//#include<time.h>
//#include<ctype.h>
//#define pi acos(-1.0)
#define P(type,x) fprintf(stdout,"%"#type"\n",x)
#define S(type,x) fscanf(stdin,"%"#type,&x)
//#define MAX(a,b) a>b?a:b
//#define MIN(a,b) a<b?a:b

int main() {
	int n;
	S(d, n);
	P(d, (n == 0 || n == 2) ? 1 : n == 1 ? 2 : 0);
	return 0;
}