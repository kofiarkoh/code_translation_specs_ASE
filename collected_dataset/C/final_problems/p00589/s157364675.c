#define _CRT_SECURE_NO_WARNINGS
//#define _USE_MATH_DEFINES
#include<stdio.h>
//#include<stdlib.h>
//#include<math.h>
#include<string.h>
//#include<time.h>
#define P(type,x) fprintf(stdout,"%"#type"\n",x)
const char code[10][10] = {"","',.!?","abcABC","defDEF","ghiGHI","jklJKL","mnoMNO"," pqrsPQRS","tuvTUV","wxyzWXYZ" };

int main() {
	int c = 0;
	char input[128],*p;
	for (;~fscanf(stdin, "%[^\n]", input)!=NULL; puts("")) {
		getchar();
		p = input;
		c = 0;
		while (*p) {
			if (*(p+1) != *p) {
				if(*p!='0') fprintf(stdout, "%c", code[*p - '0'][c]);
				c = 0;
			}
			else c++;
			if(*p=='0'&&*(p+1)=='0') putchar(' ');
			p++;
		}
	}
	return 0;
}