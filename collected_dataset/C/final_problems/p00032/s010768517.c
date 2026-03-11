#include <stdio.h>

int main(){
	int len1, len2, len3, ccount = 0, hcount = 0, hoge, hogehoge;
	while (scanf("%d%c%d%c%d", &len1, &hoge, &len2, &hogehoge, &len3) != EOF){
		if (len1*len1 + len2*len2 == len3*len3) ccount++;
		if (len1 == len2) hcount++;
	}
	printf("%d\n%d\n", ccount, hcount);
	return 0;
}