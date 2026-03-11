#include<stdio.h>
#include<string.h>
#define P(type,x) fprintf(stdout,"%"#type"\n",x)
#define S(type,x) fscanf(stdin,"%"#type,&x)
//#define MAX(a,b) a>b?a:b
//#define MIN(a,b) a<b?a:b
const char code[10][10] = { "","',.!?","abcABC","defDEF","ghiGHI","jklJKL","mnoMNO","pqrsPQRS","tuvTUV","wxyzWXYZ" };

int main() {
	int c = 0, a;
	char input[10001], *p;
	for (; ~fscanf(stdin, "%[^\n]*s", input) != NULL; puts("")) {
		getchar();
		p = input;
		c = 0;
		while (*p) {
			if (*(p + 1) != *p) {
				int i = *p - '0';
				a = strlen(code[i]);
				if (*p != '0') fprintf(stdout, "%c", code[i][c%a]);
				c = 0;
			}
			else c++;
			if (*p == '0'&&*(p + 1) == '0') putchar(' ');
			p++;
		}
	}
	return 0;
}