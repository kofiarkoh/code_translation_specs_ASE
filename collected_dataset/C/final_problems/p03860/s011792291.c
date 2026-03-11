#include<stdio.h>
#include<ctype.h>

int main() {
	//int i=0;
	char a[101],b[101],c[101];

	scanf("%s %s %s",a,b,c);
	char out[4];

	snprintf(out,4,"%c%c%c",a[0],b[0],c[0]);

	/*for (i = 0; i < 101;i++) {
		if(isupper(str[i]) !=0)*/
		printf("%s",out);
	//}

	return 0;

}